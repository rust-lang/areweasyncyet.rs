use crate::data::{Issue, IssueId};
use crate::query::{issue_or_pr, issues_with_label, Repo};
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;

#[derive(Default, Deserialize, Serialize)]
pub struct IssueData {
    #[serde(with = "hashmap_serialization")]
    pub labels: HashMap<(Repo, String), Vec<IssueId>>,
    #[serde(with = "hashmap_serialization")]
    pub issues: HashMap<(Repo, IssueId), Issue>,
}

/// Fetch and fill into `data` when corresponding information does not exist.
/// Nothing would be updated if everything is available.
///
/// Returns whether anything is updated when succeeded.
pub fn fetch_data(
    build_req: impl Fn() -> RequestBuilder,
    labels: &[(Repo, &str)],
    issues: &[(Repo, IssueId)],
    data: &mut IssueData,
) -> Result<bool, Box<dyn Error>> {
    let mut updated = false;
    for (repo, label) in labels.iter() {
        let key = (repo.clone(), label.to_string());
        if data.labels.contains_key(&key) {
            continue;
        }
        let issues = issues_with_label::query(&build_req, repo, label)?;
        let issues = issues
            .iter()
            .map(|issue| {
                let id = issue.number;
                data.issues.insert((repo.clone(), id), issue.clone());
                id
            })
            .collect();
        data.labels.insert(key, issues);
        updated = true;
    }
    for (repo, issue_id) in issues.iter() {
        let key = (repo.clone(), *issue_id);
        if data.issues.contains_key(&key) {
            continue;
        }
        let issue = issue_or_pr::query(&build_req, repo, *issue_id)?;
        data.issues.insert(key, issue);
        updated = true;
    }
    Ok(updated)
}

mod hashmap_serialization {
    use serde::de::{Deserialize, Deserializer, SeqAccess, Visitor};
    use serde::ser::{Serialize, SerializeSeq, Serializer};
    use std::cmp::Eq;
    use std::collections::HashMap;
    use std::fmt;
    use std::hash::Hash;
    use std::marker::PhantomData;

    pub fn serialize<K, V, S>(map: &HashMap<K, V>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        K: Eq + Hash + Serialize,
        V: Serialize,
    {
        let mut seq = serializer.serialize_seq(Some(map.len()))?;
        for item in map.iter() {
            seq.serialize_element(&item)?;
        }
        seq.end()
    }

    pub fn deserialize<'de, K, V, D>(deserializer: D) -> Result<HashMap<K, V>, D::Error>
    where
        D: Deserializer<'de>,
        K: Eq + Hash + Deserialize<'de>,
        V: Deserialize<'de>,
    {
        deserializer.deserialize_seq(HashMapVisitor(PhantomData))
    }

    struct HashMapVisitor<K, V>(PhantomData<fn() -> HashMap<K, V>>);

    impl<'de, K, V> Visitor<'de> for HashMapVisitor<K, V>
    where
        K: Deserialize<'de> + Eq + Hash,
        V: Deserialize<'de>,
    {
        type Value = HashMap<K, V>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a list of key-value pairs")
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            let mut map = HashMap::with_capacity(seq.size_hint().unwrap_or(0));
            while let Some((key, value)) = seq.next_element()? {
                map.insert(key, value);
            }
            Ok(map)
        }
    }
}
