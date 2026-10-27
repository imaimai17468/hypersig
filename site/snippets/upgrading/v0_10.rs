# use hypersig::{Page, PathSegment, Route, Signals};
# use serde::{Deserialize, Serialize};
#
# #[derive(Signals, Serialize, Deserialize)]
# struct Org {}
#
# async fn org((): (), _: PathSegment) -> Option<Page<Org>> { None }
hypersig::routes! {
    fn pages(Router<()>);
//-    const ORG: Route<String> = "/orgs/{org}" => org;
    const ORG: Route<PathSegment> = "/orgs/{org}" => org; //+
}

# let name = "acme";
//-let url = ORG.url(name.to_owned());
let url = ORG.url(PathSegment::parse(name)?); //+
# assert_eq!(url.as_str(), "/orgs/acme");
# let _ = pages;
# Ok::<(), hypersig::PathSegmentError>(())
