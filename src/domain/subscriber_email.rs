use validator::ValidateEmail;

#[derive(Debug)]
pub struct SubscriberEmail(String);

impl SubscriberEmail{
    pub fn parse(s: String) ->Result<SubscriberEmail, String>{
        if s.validate_email() {
            Ok(Self(s))
        }else {
            Err(format!("{} is not a valid subscriber email.", s))
        }
    }
}

impl AsRef<str> for SubscriberEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod test{
    use super::SubscriberEmail;
    use claims::assert_err;
    use fake::faker::internet::en::SafeEmail;
    // use fake::rand::RngCore;
    use fake::Fake;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn empty_string_is_rejected(){
        let email = "".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }

    #[test]
    fn email_missing_at_symbol_is_rejected(){
        let email = "ursuladomain.com".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }

    #[test]
    fn email_missing_subject_is_rejected(){
        let email = "@domain.com".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }

    // #[test]
    // fn valid_emails_are_parsed_successfully(){
    //     // this is known as property based test
    //     let email = SafeEmail().fake();
    //     claims::assert_ok!(SubscriberEmail::parse(email));
    // }
    
    #[derive(Debug, Clone)]
    struct  ValidateEmailFixture(pub String);
    
    impl quickcheck::Arbitrary for ValidateEmailFixture {
        // ! this won't work
        // fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
        //     let email = SafeEmail().fake_with_rng(g);
        //     Self(email)
        // }

        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let mut rng = StdRng::seed_from_u64(u64::arbitrary(g));
            // let mut rng = StdRng::seed_from_u64();
            let email = SafeEmail().fake_with_rng(&mut rng);

            Self(email)
        }

    }
    #[quickcheck_macros::quickcheck]
    fn valid_emails_are_parsed_successfully(valid_email: ValidateEmailFixture)->bool{
        dbg!(&valid_email.0);
        SubscriberEmail::parse(valid_email.0).is_ok()
    }
}