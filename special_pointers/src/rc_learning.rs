use std::rc::Rc;

struct Config{
    _db_url: String,
    _api_key: String 
}

struct DbService{
    _config: Rc<Config>
}

struct ApiService{
    _config: Rc<Config>
}

pub fn initialize(){
    let config = Rc::new(Config{
        _db_url: "Some database url".to_string(),
        _api_key: "Some api key".to_string()
    });
    let _db_service = DbService{_config: Rc::clone(&config)};
    let _api_service = ApiService{_config: Rc::clone(&config)};
    println!("{:?}",Rc::strong_count(&config));
}