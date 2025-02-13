
#[allow(non_snake_case)]
#[derive(Debug,Clone)]
pub struct EnvConfig{
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_expires_in: String,
    pub jwt_maxage:i32,
}

impl EnvConfig{
    pub fn init()->EnvConfig{
        let database_url=std::env::var("DATABASE_URL")
         .expect("DATABASE_URL must be set");
        let jwt_secret=std::env::var("JWT_SECRET")
         .expect("JWT_SECRET must be set");
        let jwt_expires_in=std::env::var("JWT_EXPIRED_IN")
         .expect("JWT_EXPIRED_IN must be set");
        let jwt_maxage=std::env::var("JWT_MAXAGE")
         .expect("JWT_MAXAGE must be set");

        EnvConfig{
            database_url,
            jwt_secret,
            jwt_expires_in,
            jwt_maxage:jwt_maxage.parse::<i32>().unwrap(),
        }
    }
}