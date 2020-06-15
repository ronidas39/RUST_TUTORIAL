pub fn nth(n: u32) -> u32 {            

    if (n==0){
        2
    }
    else if (n==1){
        3
    }
    else{
         let mut count:u32=1;
         let mut np:u32=3;

         while(count<=n){

            let mut temp:u32=2;
            let mut p:u32=0;
             while(temp<np/2){
                 if (np%temp==0){
                     p=1;
                     break;
                 }
                 temp=temp+1;

             }
             if (p==1){
                 count=count-1;
             }
             if (count<n){
                 np=np+2;
             }
             count=count+1;

         }


    np
    }   
   
}
