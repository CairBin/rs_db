use rs_db::file_manager::{self, FileManager};
use rs_db::logger::{
    self, Logger, U64_BYTES
};
use rs_db::file_manager::{
    page::Page
};


fn make_record(s:&str, n:u64)->Page{
    let pos = Page::max_length_for_string(s);

    let mut page = Page::new_as_size(pos+U64_BYTES);
    page.set_string(0, s).unwrap();
    page.set_u64(pos, n).unwrap();

    return page;
}

fn create_record(log: &mut Logger, start:u64, end:u64){
    let mut i = start;
    while i<= end{
        let filename = format!("record{}", i);
        let page = make_record(&filename, i);
        log.append(page.contents()).unwrap();
        i+=1;
    }
}

#[test]
fn test_logger(){
    let fm = FileManager::new("log_test", 400).unwrap().to_mutex();
    let mut log = Logger::new(&fm, "log_file").unwrap();

    create_record(&mut log, 1, 35);

    let iter = log.iter().unwrap();

    for l in iter{
        let page = Page::new_as_bytes(&l);
        let s = page.get_string(0).unwrap();
        println!("{}", s);
    }
}