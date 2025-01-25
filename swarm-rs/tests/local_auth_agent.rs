use swarm_rs::{
    swarm::Swarm,
    utils::{file_io, json_io},
    web::{AuthAgent, NewUser, PasswordUpdate, UserAuth, UserCredentials, UserToken},
};

#[tokio::test]
pub async fn auth_agent() {
    let auth: AuthAgent = json_io::load("test-data/agents/auth.json").unwrap();
    file_io::remove_file("test-data/out/users.json");
    let mut swarm = Swarm::new();
    swarm.register_agent("auth", auth);

    let new_user = NewUser::new("admin", "p4ssw0rd", "Admin", "", vec!["Admin"]);

    let output = swarm.execute("auth.register_user", &new_user).await;
    println!("{}", output);

    let payload = UserCredentials {
        login: "admin".to_string(),
        password: "p4ssw0rd".to_string(),
    };

    let output = swarm.execute("auth.login", &payload).await;
    println!("{}", output);
    assert!(output.is_success());

    let user_auth: UserAuth = output.get_payload();

    let payload = UserCredentials {
        login: "admin".to_string(),
        password: "password".to_string(),
    };

    let output = swarm.execute("auth.login", &payload).await;
    println!("{}", output);
    assert!(!output.is_success());

    let payload = UserToken {
        token: user_auth.token.to_string(),
    };

    let output = swarm.execute("auth.refresh_token", &payload).await;
    println!("{}", output);
    assert!(output.is_success());

    let payload = PasswordUpdate {
        token: user_auth.token.to_string(),
        old_password: "p4ssw0rd".to_string(),
        new_password: "new_p4ssw0rd".to_string(),
    };

    let output = swarm.execute("auth.update_password", &payload).await;
    println!("update_password : {}", output);
    assert!(output.is_success());

    let payload = UserCredentials {
        login: "admin".to_string(),
        password: "p4ssw0rd".to_string(),
    };

    let output = swarm.execute("auth.login", &payload).await;
    println!("{}", output);
    assert!(!output.is_success());


    let payload = UserCredentials {
        login: "admin".to_string(),
        password: "new_p4ssw0rd".to_string(),
    };

    let output = swarm.execute("auth.login", &payload).await;
    println!("{}", output);
    assert!(output.is_success());
}
