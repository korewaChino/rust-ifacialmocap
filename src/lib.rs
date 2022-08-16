use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use regex::Regex;
#[derive(Serialize, Deserialize, Debug)]
pub struct Value {
    #[serde(flatten)]
    pub int_keys: HashMap<String, i32>,
    #[serde(flatten)]
    pub xyz_keys: HashMap<String, [f32; 3]>,
    #[serde(flatten)]
    pub coord_keys: HashMap<String, ([f32; 3],[f32; 3])>

}


impl Value {
    fn new() -> Self {
        Value {
            int_keys: HashMap::new(),
            xyz_keys: HashMap::new(),
            coord_keys: HashMap::new()
        }
    }
    pub fn parse_str<S: Into<String>>(s: S) -> Result<Self, String> {
        let mut value = Value::new();

        let regex = Regex::new(r"[^\|?]*").unwrap();
        let str = s.into();
        let capt = regex.captures_iter(&str).collect::<Vec<_>>();

        let error_message = "Invalid value string: ".to_string();

        for cap in capt {
            cap.iter().for_each(|x| {
                let x = x.unwrap().as_str();
                let key = if x.contains('#') {
                    x.split_once('#').unwrap_or(("", "0")).0
                } else {
                    //println!("{:?}", x.split_once("-"));
                    x.split_once('-').unwrap_or(("", "0")).0
                };

                let val_str = if x.contains('#') {
                    // value is array of string
                    let val = x.split_once('#').unwrap_or(("", "0")).1;
                    val.split(',').map(|x| x.to_string()).collect::<Vec<String>>()
                } else {
                    // value is a single value
                    vec![x.split_once('-').unwrap_or(("", "0")).1.to_string()]
                };

                if val_str.len() == 1 {
                    // parse value as integer
                    let val = val_str[0].parse::<i32>().map_err(|_| error_message.clone()).unwrap();
                    value.int_keys.insert(key.to_string(), val);
                } else if val_str.len() == 3 {
                    // parse value as xyz
                    let val = [
                        val_str[0].parse::<f32>().map_err(|_| error_message.clone()).unwrap(),
                        val_str[1].parse::<f32>().map_err(|_| error_message.clone()).unwrap(),
                        val_str[2].parse::<f32>().map_err(|_| error_message.clone()).unwrap()
                    ];
                    value.xyz_keys.insert(key.to_string(), val);
                } else if val_str.len() == 6 {
                    // parse value as coord
                    let val = ([
                        val_str[0].parse::<f32>().map_err(|_| error_message.clone()).unwrap(),
                        val_str[1].parse::<f32>().map_err(|_| error_message.clone()).unwrap(),
                        val_str[2].parse::<f32>().map_err(|_| error_message.clone()).unwrap()
                    ], [
                        val_str[3].parse::<f32>().map_err(|_| error_message.clone()).unwrap(),
                        val_str[4].parse::<f32>().map_err(|_| error_message.clone()).unwrap(),
                        val_str[5].parse::<f32>().map_err(|_| error_message.clone()).unwrap()
                    ]);
                    value.coord_keys.insert(key.to_string(), val);
                }
            } );
        }
        value.int_keys.remove("");

        Ok(value)
    }

    pub fn parse_str_lossy<S: Into<String>>(s: S) -> Result<Self, String> {
        let mut value = Value::new();
        let regex = Regex::new(r"[^\|?]*").unwrap();
        let str = s.into();
        let capt = regex.captures_iter(&str).collect::<Vec<_>>();

        for cap in capt {
            cap.iter().for_each(|x| {
                let x = x.unwrap().as_str();
                let key = if x.contains('#') {
                    x.split_once('#').unwrap_or(("", "0")).0
                } else {
                    //println!("{:?}", x.split_once("-"));
                    x.split_once('-').unwrap_or(("", "0")).0
                };

                let val_str = if x.contains('#') {
                    // value is array of string
                    let val = x.split_once('#').unwrap_or(("", "0")).1;
                    val.split(',').map(|x| x.to_string()).collect::<Vec<String>>()
                } else {
                    // value is a single value
                    vec![x.split_once('-').unwrap_or(("", "0")).1.to_string()]
                };

                if val_str.len() == 1 {
                    // parse value as integer
                    let val = val_str[0].parse::<i32>().unwrap_or(0);
                    value.int_keys.insert(key.to_string(), val);
                } else if val_str.len() == 3 {
                    // parse value as xyz
                    let val = [
                        val_str[0].parse::<f32>().unwrap_or(0.0),
                        val_str[1].parse::<f32>().unwrap_or(0.0),
                        val_str[2].parse::<f32>().unwrap_or(0.0),
                    ];
                    value.xyz_keys.insert(key.to_string(), val);
                } else if val_str.len() == 6 {
                    // parse value as coord
                    let val = ([
                        val_str[0].parse::<f32>().unwrap_or(0.0),
                        val_str[1].parse::<f32>().unwrap_or(0.0),
                        val_str[2].parse::<f32>().unwrap_or(0.0),
                    ], [
                        val_str[3].parse::<f32>().unwrap_or(0.0),
                        val_str[4].parse::<f32>().unwrap_or(0.0),
                        val_str[5].parse::<f32>().unwrap_or(0.0),
                    ]);
                    value.coord_keys.insert(key.to_string(), val);
                }
            } );
        }
        value.int_keys.remove("");

        Ok(value)
    }
}


#[cfg(test)]
mod tests {
    use crate::Value;

    static TEST_DATA: &str = "mouthSmile_R-0|eyeLookOut_L-0|mouthUpperUp_L-11|eyeWide_R-0|mouthClose-8|mouthPucker-4|mouthRollLower-9|eyeBlink_R-7|eyeLookDown_L-17|cheekSquint_R-11|eyeBlink_L-7|tongueOut-0|jawRight-0|eyeLookIn_R-6|cheekSquint_L-11|mouthDimple_L-10|mouthPress_L-4|eyeSquint_L-11|mouthRight-0|mouthShrugLower-9|eyeLookUp_R-0|eyeLookOut_R-0|mouthPress_R-5|cheekPuff-2|jawForward-11|mouthLowerDown_L-9|mouthFrown_L-6|mouthShrugUpper-26|browOuterUp_L-4|browInnerUp-20|mouthDimple_R-10|browDown_R-0|mouthUpperUp_R-10|mouthRollUpper-8|mouthFunnel-12|mouthStretch_R-21|mouthFrown_R-13|eyeLookDown_R-17|jawOpen-12|jawLeft-0|browDown_L-0|mouthSmile_L-0|noseSneer_R-18|mouthLowerDown_R-8|noseSneer_L-21|eyeWide_L-0|mouthStretch_L-21|browOuterUp_R-4|eyeLookIn_L-4|eyeSquint_R-11|eyeLookUp_L-0|mouthLeft-1|=head#-21.488958,-6.038993,-6.6019735,-0.030653415,-0.10287084,-0.6584072|rightEye#6.0297494,2.4403017,0.25649446|leftEye#6.034903,-1.6660284,-0.17520553|";
    static INVALID_DATA: &str = "mouthLeft-0|browInnerUp-6|mouthLowerDown_L-4|mouthDimple_R-2|mouthFunnel-5|eyeSquint_L-12|browOuterUp_L-0|mouthUpperUp_L-4|mouthFrown_R-2|eyeLookOut_R-0|mouthShrugUpper-11|eyeSquint_R-12|eyeLookDown_R-15|mouthRollLower-6|eyeLookDown_L-16|cheekSquint_L-9|mouthSmile_L-0|mouthRight-0|mouthDimple_L-2|jawRight-0|mouthPucker-24|mouthRollUpper-1|mouthPress_L-8|eyeLookOut_L-0|browDown_R-13|cheekSquint_R-8|mouthFrown_L-3|tongueOut-0|mouthPress_R-10|browDown_L-12|mouthLowerDown_R-4|eyeWide_L-2|cheekPuff-7|mouthSmile_R-0|eyeLookIn_L-0|eyeLookUp_L-0|jawForward-3|jawLeft-4|noseSneer_L-13|jawOpen-2|mouthStretch_R-8|eyeLookUp_R-0|mouthClose-4|eyeWide_R-2|eyeBlink_L-2|eyeLookIn_R-12|noseSneer_R-9|eyeBlink_R-2|mouthUpperUp_R-4|browOuterUp_R-0|mouthStretch_L-9|mouthShrugLower-14|hapihapi-0|=head#25.409164,-5.085786,3.8090365,0.052303925,0.2366666,-0.0259732|rightEye#5.2707267,4.227702,0.41178665|leftEye#5.300755,0.32921365,0.03218361|-0.67254096|||||3|";

    #[test]
    fn it_works() {
        let a = Value::parse_str(TEST_DATA);
        assert!(a.is_ok());
    }
    #[test]
    fn lossy_test() {
        let a = Value::parse_str_lossy(INVALID_DATA);
        assert!(a.is_ok());
    }
}
