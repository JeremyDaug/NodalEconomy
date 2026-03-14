pub mod package;
pub mod good;

#[cfg(test)]
mod test {
    mod good {
        mod get_tag_should {
            use crate::good::{Good, GoodTag};

            #[test]
            fn get_a_tag_and_return_it() {
                let good1 = Good::new(0, String::from("Test1"))
                    .with_tags(vec![
                        GoodTag::Deconstructable { reduction: 0.4, deconstruction_time: 10.0, reconstruction_time: 10.0 }
                    ]);
                if let Some(GoodTag::Deconstructable { reduction, deconstruction_time, reconstruction_time }) = good1.get_tag(&GoodTag::deconstructable()) {
                    assert_eq!(reduction, 0.4);
                    assert_eq!(deconstruction_time, 10.0);
                    assert_eq!(reconstruction_time, 10.0);
                } else {
                    assert!(false, "Did not return tag correctly!");
                }
            }
        }

        mod decay_good_should {
            use std::collections::HashMap;

            use crate::good::Good;

            #[test]
            fn correctly_calculate_decay() {
                let mut result1 = HashMap::new(); 
                result1.insert(1, 10.0);
                result1.insert(2, 0.5);
                let good1 = Good::new(0, String::from("Test1"))
                    .with_decay(0.5, result1);

                let result = good1.decay_good(10.0);
                assert_eq!(result.len(), 2);
                assert_eq!(result[&1], 100.0);
                assert_eq!(result[&2], 5.0);
            }
        }
    }
}