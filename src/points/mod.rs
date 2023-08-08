use glam::Vec3;
use usd::pxr::usd::{Stage,stage_desc,Prim,TimeCode};
use usd::pxr::{usd_geom,tf,vt};

pub fn read_usd_pts(file_path: String)->Vec<Vec3>{
    let mut pts: Vec<Vec3> = Vec::new(); 
    let stage = Stage::open(stage_desc::Open {
        file_path: file_path.as_str(),
        load: None,
    }).expect("could not load stage");
    
    for prim in stage.traverse().unwrap().iter() {
        let prim_type = prim.get_type_name().get_text().unwrap();
        println!("prim type: {:?}",prim_type);
        let pts_read = get_attr_ArrayVec3f(&prim, "points");
        match pts_read {
            Some(data)=> {let mut tmp = data.0.clone(); pts.append(&mut tmp)},
            None => {}
        };
    }
    pts.sort_by(|a, b| a.y.partial_cmp(&b.y).unwrap());
    pts
}

fn get_attr_ArrayVec3f(prim: &Prim, name: &str)-> Option< (Vec<Vec3>, String) >{
    if prim.has_attribute(&tf::Token::try_from(name).unwrap()){
        let attr = prim.get_attribute(&tf::Token::try_from(name).unwrap());
        if attr.has_value(){
            use vt::VtArray as _;
            let mut array = vt::ArrayVec3f::new();
            let mut _value = vt::Value::from(&array);
            attr.get(&mut _value, TimeCode::default());
            let points: &vt::ArrayVec3f = _value.as_ref();

            //primvar
            let pv = usd_geom::Primvar::new(&attr);
            let pv_interpolation = pv.as_ref().get_interpolation().get_text().unwrap().to_string();

            let mut out: Vec<Vec3> = Vec::with_capacity(points.size());

            for i in 0 ..points.size(){
                out.push(points[i].into());
            };
            Some( (out,pv_interpolation) )
        }
        else {
            None
        }
    }
    else{
        None
    }
}
