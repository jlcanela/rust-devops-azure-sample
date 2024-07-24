use std::fs;

use super::action::*;

// use cedar_policy::PrincipalConstraint::{Any, Eq, In, Is, IsIn};
use cedar_policy::{
    Authorizer, Context, Decision, Entities, /*Entity,*/ EntityId, EntityTypeName, EntityUid, /*Policy,*/
    /*PolicyId,*/ PolicySet, Request, /*Response, RestrictedExpression,*/ Schema, /*SlotId, Template,*/
    //ValidationMode, ValidationResult, Validator,
};
//use std::collections::{HashMap, HashSet};
use std::str::FromStr;


#[derive(Debug)]
pub struct Permission {
    policies: PolicySet,
    _schema: Schema
}

impl Default for Permission {

    fn default() -> Self {

        let policies = fs::read_to_string("cedar-policies/projects/policies.cedar").expect("Should have been able to read the 'policies' file");
        let policies = PolicySet::from_str(&policies).unwrap();
        let schema = fs::read_to_string("cedar-policies/projects/projects.cedarschema").expect("Should have been able to read the 'schema' file");
        let (schema, warnings) = Schema::from_str_natural(&schema).unwrap();

        for w in warnings {
            println!("{:?}", w);
        }

        Self {
            policies,
            _schema: schema
        }
     }
}

impl Permission {
    
    fn principal(token: Option<String>) -> EntityUid {
        match token {
            None => {
                let p_eid = EntityId::from_str("anonymous").unwrap(); // does not go through the parser
                let p_name: EntityTypeName = EntityTypeName::from_str("User").unwrap(); // through parse_name(s)
                EntityUid::from_type_name_and_id(p_name, p_eid)
            }    
            Some(_token) => {
                let p_eid = EntityId::from_str("somebody").unwrap(); // does not go through the parser
                let p_name: EntityTypeName = EntityTypeName::from_str("User").unwrap(); // through parse_name(s)
                EntityUid::from_type_name_and_id(p_name, p_eid)
            }
    
        } 
    }

    fn project() -> EntityUid {
        let r_eid = EntityId::from_str("1").unwrap(); // does not go through the parser
        let r_name: EntityTypeName = EntityTypeName::from_str("Project").unwrap(); // through parse_name(s)
        EntityUid::from_type_name_and_id(r_name, r_eid)
    }

    fn create_static_entities() -> Entities {
        let e = r#"[
            {
                "uid": {
                    "type": "Group",
                    "id": "AllUsers"
                },
                "attrs": {},
                "parents": []
            },
            {
                "uid": {
                    "type": "Group",
                    "id": "AllProjects"
                },
                "attrs": {},
                "parents": []
            }
        ]"#;
        Entities::from_json_str(e, None).expect("entity error")
    }

    fn create_entities_json() -> Entities {
        let e = r#"[
    {
        "uid": {
            "type": "User",
            "id": "anonymous"
        },
        "attrs": {
        },
        "parents": [
            {
                "type": "Group",
                "id": "AllUsers"
            }
        ]
    },
    {
        "uid": {
            "type": "User",
            "id": "somebody"
        },
        "attrs": {
        },
        "parents": [
            {
                "type": "Group",
                "id": "AllUsers"
            }
        ]
    },
    {
        "uid": {
            "type": "Project",
            "id": "1"
        },
        "attrs": {
        },
        "parents": [
            {
                "type": "Group",
                "id": "AllProjects"
            }
        ]
    }
]"#;
    
        Entities::from_json_str(e, None).expect("entity error")
    }

    pub fn is_authorized<T>(&self, token: Option<String>, action: Action, _resource: &T) -> bool {
        let authorizer = Authorizer::new();

        let p = Permission::principal(token);
        let a: EntityUid = action.into();
        let r = Permission::project();

        let request: Request = Request::new(Some(p), Some(a), Some(r), Context::empty(), None).unwrap();

        let mut entities = Entities::empty();
        entities = entities.add_entities(Permission::create_static_entities(), None).unwrap();
        entities = entities.add_entities(Permission::create_entities_json(), None).unwrap();

        let ans = authorizer.is_authorized(&request, &self.policies, &entities);

        for reason in ans.diagnostics().reason() {
            //print all the annotations
            for (key, value) in self.policies.policy(reason).unwrap().annotations() {
                println!("PolicyID: {}\tKey:{} \tValue:{}", reason, key, value);
            }
        }

        ans.decision() == Decision::Allow
    }

}