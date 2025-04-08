#[cfg(test)]
mod tests{

    //use super::*;  load outside mod
    
    /*
        assert!(x) : x=false->panic , don't output value of x
        assert_eq!(x,y) : will output values of x and y
        assert_ne!(x,y)

        define error information by self 
        assert!(x,"xxxxx{}",parameter)
     */

    #[test]
    fn test(){
        
    }

    //should_panic : happen panic-->pass
    #[test]
    #[should_panic]
    //#[should_panic(expected="xxxxx")] -->happen panic and panic contain error infor "xxxxx"-->pass
    fn should_panic(){

    }

    //use Result<T,E> in rest
    #[test]
    fn result_test()->Result<(),String>{
        if 2+2==4{
            Ok(())
        }else{
            Err(String::from("two plus two does not equal four"))
        }
    }

}
