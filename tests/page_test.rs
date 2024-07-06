use rs_db::file_manager;


#[test]
fn test_set_and_get_u64() {
    let mut page = file_manager::page::Page::new_as_size(256);
    page.set_u64(23,1234).unwrap();
    assert_eq!(page.get_u64(23).unwrap(), 1234);
}

#[test]
fn test_set_and_get_bytes() {
    let mut page = file_manager::page::Page::new_as_size(256);
    let bytes:[u8;4] = [1,2,3,4];

    page.set_bytes(23,&bytes).unwrap();
    assert_eq!(page.get_bytes(23).unwrap(), &bytes);
}

#[test]
fn test_set_and_get_string(){
    let mut page = file_manager::page::Page::new_as_size(256);
    let string = "hello world";

    page.set_string(23,string).unwrap();
    assert_eq!(page.get_string(23).unwrap(), string);
}

#[test]
fn test_max_length_for_string(){
    let string = "你好，世界";
    let len = string.as_bytes().len() + 8;
    assert_eq!(file_manager::page::Page::max_length_for_string(string), len);
}


#[test]
fn test_contents(){
    let bytes:[u8;5] = [1,2,3,4,5];
    let mut page = file_manager::page::Page::new_as_bytes(&bytes);

    assert_eq!(page.contents(), bytes);
}