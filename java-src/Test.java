import java.io.File;

public class Test {

    public static native String[] toRustFromStringArray(String[] arr, int length);
    
    public static void main(String[] args){
        String[] javaArray = new String[] {
            "a","b","c","d"
        };

        File f = new File(".");
        String path = f.getAbsolutePath().replace(".", "target\\debug\\java_rust.dll");

        if( new File(path).exists()){
            System.out.println("파일이 존재함!!");
        }

        System.load(path);
        
        var tmp = toRustFromStringArray(javaArray, javaArray.length);

        System.out.println("러스트 함수 호출 후");

        for (int i =0; i<tmp.length; i++) {
            System.out.println("From Rust : " + tmp[i]);
        }

        
    }

}