
use jni::{objects::JString, sys::{jint, jobjectArray}};
use jni::objects::JClass;
use jni::JNIEnv;

#[no_mangle]
pub extern  fn Java_Test_toRustFromStringArray(
    env: JNIEnv, 
    _:  JClass,  
    array: jobjectArray ,
    length: jint)  -> jobjectArray
{
    println!("rust function start");

    // 배열 크기 가져오는건데..
    
    if array.is_null()  {
        panic!("array is nul!!!!");
    }

    let size = length;

    println!("len : {} ", size );

    for i in 0..size {
        let item = match  env.get_object_array_element(array, i) {
            Ok(data) => { data}
            Err(err) => { panic!("err : {}", err) }
        };

        let jstr = JString::from(item);
        let java_str = env.get_string(jstr).unwrap();
        let string_dat = java_str.to_str().unwrap();

        println!("from java : {}", string_dat);
    }

    let rust_array = vec!["1","2","3","4"];

    let fir_ele = env.new_string("").unwrap();

    let size = rust_array.len() as i32;

    let string_class = env.find_class("java/lang/String").unwrap();
    
    let obj_array = env.new_object_array(size, 
        string_class, fir_ele).unwrap();

    for i in 0..size {
        let item =  env.new_string( rust_array[i as usize]).unwrap();
        env.set_object_array_element(obj_array, i,  item ).unwrap();
    }
    
    return obj_array;
}