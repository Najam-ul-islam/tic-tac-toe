mod user_input;
pub mod draw_board{
    pub fn sketch(){
        let mut a =vec![["_","_","_"],["_","_","_"],["_","_","_"]];
        let  a00 = a[0][0]; 
        let  a01 = a[0][1];
        let  a02 = a[0][2];
    //================================
        let  a10 = a[1][0]; 
        let  a11 = a[1][1];
        let  a12 = a[1][2];
    //================================
        let  a20 = a[2][0]; 
        let  a21 = a[2][1];
        let  a22 = a[2][2];
    //================================
            println!("   |     |  ");
            println!("  {}|  {}  |{}",a00,a01,a02);
            println!("---+-----+---");
            println!("  {}|  {}  |{}",a10,a11,a12);
            println!("---+-----+---");
            println!("  {}|  {}  |{}",a20,a21,a22);
            println!("   |     |  ");
        if user_input::player::player_1 == a00{a.push('X');}
        else if user_input::player::player_1 == a01{a.push('X');}
        else{println!("OK");}
    }
}