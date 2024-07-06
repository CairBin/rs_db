use rs_db::file_manager;
use rs_db::file_manager::page::Page as Page;
use rs_db::file_manager::block::BlockId as BlockId;
use rs_db::file_manager::FileManager as FileManager;

#[test]
fn test_file_manager(){
    let fm = file_manager::FileManager::new("file_test", 400).unwrap();
    let block = file_manager::block::BlockId::new("test_file", 2);

    let s = "hello world";
    let mut page1 = file_manager::page::Page::new_as_size(fm.get_block_size() as usize);
    page1.set_string(88, s).unwrap();
    let size = file_manager::page::Page::max_length_for_string(s);
    page1.set_u64(88+size, 123).unwrap();

    let page2 = file_manager::page::Page::new_as_size(fm.get_block_size() as usize);
    
    let page2 = page2.to_mutex();
    let fm = fm.to_mutex();
    let page1 = page1.to_mutex();

    FileManager::write(&fm, &block, &page1).unwrap();
    FileManager::read(&fm, &block, &page2).unwrap();
    
    assert_eq!(page1.lock().unwrap().get_string(88).unwrap(), s);
    assert_eq!(page2.lock().unwrap().get_u64(88+size).unwrap(), 123);

}