use lib::{
    domains::user::{Email, Password, UserID, Username},
    utils::password::hash_password,
};
use sqlx::PgPool;
use uuid::Uuid;

pub struct TestUser {
    pub id: UserID,
    pub username: Username,
    pub password: Password,
    pub email: Email,
}

pub async fn create_test_user(pool: &PgPool) -> TestUser {
    let user_id = Uuid::new_v4();
    let test_user = TestUser {
        id: UserID(user_id),
        email: Email(format!("{}@mail.com", Uuid::new_v4().to_string())),
        username: Username(Uuid::new_v4().to_string()),
        password: Password(Uuid::new_v4().to_string()),
    };

    let password_hash: Password = hash_password(test_user.password.clone()).await.unwrap();

    sqlx::query!(
        r#"
            INSERT INTO public.users (
                id, email, username, password
            ) VALUES (
                $1, $2, $3, $4
            );
        "#,
        &user_id,
        test_user.email.as_ref(),
        test_user.username.as_ref(),
        password_hash.as_ref()
    )
    .execute(pool)
    .await
    .unwrap();

    test_user
}
