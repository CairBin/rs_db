mod file_manager;

fn test_array(arr:&[i32])->Result<(),Box<dyn std::error::Error>>{
    arr[1];
    Ok(())
}

fn main() {
    // let mut t = file_manager::page::Page::new_as_size(1024);
    // let val = t.get_u64(1017).unwrap();
    // println!("{}",val);

    let arr:[i32;3] = [1,2,3];
    println!("{:?}", test_array(&arr));
}
