use std::{cell::RefCell, rc::Rc};

struct Logger{
    data: RefCell<Vec<String>>
}

struct DbService{
    logger: Rc<Logger>
}

struct ApiService{
    logger: Rc<Logger>
}


pub fn learn(){
    let logger = Rc::new(Logger{
        data: RefCell::new(vec![])
    });

    let db_service = DbService{logger: Rc::clone(&logger)};
    let api_service = ApiService{logger: Rc::clone(&logger)};

    db_service.logger.data.borrow_mut().push("log from db service".to_string());
    api_service.logger.data.borrow_mut().push("log from api service".to_string());

    for log in logger.data.borrow().iter(){
        println!("{}",log);
    }
}