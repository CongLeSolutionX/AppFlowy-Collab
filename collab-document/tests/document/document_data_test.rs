use collab::core::collab::MutexCollab;
use collab::core::origin::CollabOrigin;
use collab_document::document::Document;
use collab_document::document_data::default_document_data;
use std::sync::Arc;

#[tokio::test]
async fn get_default_data_test() {
  let data = default_document_data();
  assert!(!data.page_id.is_empty());
  assert!(!data.blocks.is_empty());
  assert!(!data.meta.children_map.is_empty());
  assert!(data.meta.text_map.is_some());
  assert!(data.meta.text_map.is_some());
  assert_eq!(data.meta.text_map.unwrap().len(), 1);

  let data = default_document_data();
  println!("{:?}", data);
  assert!(!data.page_id.is_empty());
  assert_eq!(data.blocks.len(), 2);
  assert_eq!(data.meta.children_map.len(), 2);
  assert!(data.meta.text_map.is_some());
  assert_eq!(data.meta.text_map.unwrap().len(), 1);
}

#[tokio::test]
async fn validate_document_data() {
  let document_data = default_document_data();
  let collab = Arc::new(MutexCollab::new(CollabOrigin::Empty, "1", vec![], false));
  let _ = Document::create_with_data(collab.clone(), document_data).unwrap();
  assert!(Document::validate(&collab.lock()).is_ok());

  let collab = Arc::new(MutexCollab::new(CollabOrigin::Empty, "1", vec![], false));
  assert!(Document::validate(&collab.lock()).is_err())
}
