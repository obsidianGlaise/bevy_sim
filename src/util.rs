use std::fs;
use crate::components::{charge::*,point::*};

fn to_float(s: String) -> f64 { s.parse::<f64>().unwrap() }

fn field_constants(field_consts: Vec<&str>) -> (Vec<f64>,Vec<f64>) {
    let mut coeffs = (*field_consts[0]).to_string();
    coeffs.retain(|c| c != '(' && c != ')');
    let coeffs: Vec<f64> = coeffs.trim().split(',').map(|c| to_float(c.to_string())).collect(); 
    let mut pows = (*field_consts[1]).to_string();
    pows.retain(|c| c != '(' && c != ')');
    let pows: Vec<f64> = pows.trim().split(',').map(|c| to_float(c.to_string())).collect(); 
    return (coeffs,pows);
}

pub fn setup(input: String) -> (Vec<crate::components::charge::Charge>, Vec<f64>, Vec<f64>) {
    let contents = fs::read_to_string(input.trim()).expect("Should have been able to read the file");

    let input: Vec<&str> = contents.split('\n').collect();
    let (e_coeffs_temp,e_pows_temp) = field_constants(input[1].split(' ').collect());
    let (b_coeffs_temp,b_pows_temp) = field_constants(input[2].split(' ').collect());

    let mut e_field: Vec<f64> = e_coeffs_temp;
    let mut b_field: Vec<f64> = b_coeffs_temp;
    e_field.extend(e_pows_temp);
    b_field.extend(b_pows_temp);
    
    let mut particles: Vec<Charge> = vec![];
    for i in 3..input.len() {
        let info: Vec<&str> = input[i].split(' ').collect();

        let mut pos = (*info[0]).to_string();
        pos.retain(|c| c != '(' && c != ')');
        let pos: Vec<&str> = pos.split(',').collect();
        let mut vel = (*info[1]).to_string();
        vel.retain(|c| c != '(' && c != ')');
        let vel: Vec<&str> = vel.split(',').collect();

        let pos = Point::from(to_float(pos[0].to_string()),to_float(pos[1].to_string()),to_float(pos[2].to_string()));
        let vel = Point::from(to_float(vel[0].to_string()),to_float(vel[1].to_string()),to_float(vel[2].to_string()));

        let q = to_float(info[2].to_string());
        let b = info[3].to_string().trim().parse::<bool>().unwrap();
        let c_new = Charge::new(pos,vel,q,b);
        particles.push(c_new);
    }

    return (particles, e_field, b_field);
}