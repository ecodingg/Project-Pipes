pub struct ques {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub length: f64,
    pub light: bool,
    pub sound: bool,
    pub projection: bool,
}

fn ques(temp_id: i32, temp_name: String, temp_description: String, temp_length: f64, temp_light: bool, temp_sound: bool, temp_projection: bool){
    ques.id = temp_id;
    ques.name = temp_name;
    ques.description = temp_description;
    ques.length = temp_length;
    ques.light = temp_light;
    ques.sound = temp_sound;
    ques.projection = temp_projection;

    if (ques.light == True)
    {
        
    }

    if (ques.sound == True)
    {

    }

    if (ques.projection == True)
    {

    }
}