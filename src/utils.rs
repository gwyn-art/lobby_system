use std::collections::HashMap;
use validator::ValidationErrors;

pub fn errors_to_map<'s>(errors: &'s ValidationErrors) -> HashMap<& 's str, Vec<String>> {
  let mut map = HashMap::<&str, Vec<String>>::new();

  for field in errors.field_errors().keys() {
      let mut error_messages = Vec::<String>::new();

      for error in errors.field_errors().get(field).unwrap().iter() {
          error_messages.push(error.message.clone().unwrap().into_owned())
      }

      map.insert(field, error_messages);
  }

  map
}