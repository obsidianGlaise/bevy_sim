use bevy::prelude::{Component, Vec3};

use crate::components::point;
use crate::components::point::Point;

pub const MASS: f64 = 1.0;
pub const K: f64 = 1.0;
//pub const PLANCK: f64 = 1.616e-35;
pub const C: f64 = 3.0e8;

#[derive(Debug, Copy, Clone,Component)]
pub struct Charge{
    cur_pos: point::Point,
    v: point::Point,
    past_a: point::Point,
    q: f64,
    fixed: bool,
    cur_f: point::Point,
}

impl Charge {
    pub fn new(pos: Point, v: Point, q: f64, fixed: bool)  -> Charge { 
        Charge { 
            cur_pos: pos, v: v, past_a: Point::new(), 
            q: q, fixed: fixed, cur_f: Point::new()
        } 
    }

    pub fn update(&mut self, dt: f64) {
        //? x(t+dt)=x(t)+v(t)*dt+0.5*a(t)*dt^2
        //? v(t+dt)=v(t)+(a(t)+a(t+dt))*dt/2
        let acc = Point::s_div(MASS,self.cur_f);
        let l = Point::s_times(dt,self.v);
        let r = Point::s_times(0.5*dt*dt, acc);
        self.cur_pos = Point::add(vec![self.cur_pos, l, r]);
        let v_acc = Point::s_div(2.0, Point::add(vec![acc,self.past_a]));

        self.v = Point::add(vec![self.v, Point::s_times(dt, v_acc)]);
        self.past_a = acc;
    }

    pub fn get_magnitude(self) -> f64 { self.q }
    pub fn reset(&mut self) { self.cur_f = Point::new(); }

    pub fn lorentz(self, e: &Vec<f64>, b: &Vec<f64>) -> Point {
        let e_cont = Point::from(
            e[0]*f64::powf(self.x(),e[3]),
            e[1]*f64::powf(self.y(),e[4]),
            e[2]*f64::powf(self.z(),e[5])
        );

        let b_cont = Point::from(
            b[0]*f64::powf(self.x(),b[3]),
            b[1]*f64::powf(self.y(),b[4]),
            b[2]*f64::powf(self.z(),b[5])
        );
        let fx = self.q*(e_cont.x()+self.v.y()*b_cont.z()-self.v.z()*b_cont.y());
        let fy = self.q*(e_cont.y()+self.v.z()*b_cont.x()-self.v.x()*b_cont.z());
        let fz = self.q*(e_cont.z()+self.v.x()*b_cont.y()-self.v.y()*b_cont.x());
        
        return Point::from(fx,fy,fz);
    }


    pub fn abraham_lorentz(c: Charge, dt: f64) -> point::Point {
        let jerk = Point::s_div(dt, Point::add(vec![c.cur_f, c.past_a.neg()]));
        let constants = (2.0/3.0)*K*c.q*c.q/(C*C*C);        
        return Point::s_times(constants, jerk).align(c.v).neg();
    }

    pub fn coulomb(c1: Charge, c2: Charge) -> Point {
        let r_sq = f64::powf(Point::dist(c1.cur_pos,c2.cur_pos),2.0);
        let magnitude = c1.q*c2.q*K/r_sq;
        let d = Point::add(vec![c1.cur_pos,c2.cur_pos.neg()]).unit();
        return Point::s_times(magnitude, d);
    }

    pub fn add_force(&mut self, f: Point) {
        self.cur_f = Point::add(vec![self.cur_f, f]);
    }

    pub fn display_pos(self) -> String {
        return self.cur_pos.to_string();
    }

    pub fn x(self) -> f64{ self.cur_pos.x() }

    pub fn y(self) -> f64{ self.cur_pos.y() }

    pub fn z(self) -> f64{ self.cur_pos.z() }

    pub fn get_pos(self) -> Point { self.cur_pos.clone() }
    pub fn is_fixed(self) -> bool { self.fixed }
    pub fn to_vec(self) -> Vec3 { self.cur_pos.to_vec() }
}