use ansi_term::Colour;

pub const BG: Colour = Colour::RGB(5, 40, 40);

pub const FAMILY_ROLES: [&'static str; 110] = [
  "family", "parent", "nuclear family", "nuclear family member", "family member", "immediate family", "spouse", "husband", "wife",
  "father", "mother", "step-father", "step father", "stepfather", "step-mother", "step mother", "stepmother", "step-mother", "legal guardian",
  "child", "son", "daughter", "step-son", "step son", "stepson", "step-daughter", "stepdaughter", "step daughter", "sibling", "brother", "sister",
  "extended family", "grandparent", "grandfather", "grandmother", "grandson", "granddaughter", "uncle", "aunt", "cousin", "nephew", "niece",
  "family-in-law", "family in law", "father-in-law", "father in law", "mother-in-law", "mother in law", "brother in law", "sister in law", "kin",
  "kinship caregiver", "family member", "partner", "adoptive mother", "adoptive father", "birth mother", "birth father", "guardian", "adopted child",
  "adopted son", "adopted daughter", "adoptive sister", "adoptive sibling", "step sibling", "step-sibling", "stepsibling", "adoptive brother",
  "adopted brother", "adopted sister", "foster sister", "foster brother", "foster mother", "foster mom", "foster dad", "foster parent", "foster father",
  "former foster mom", "former foster mother", "former foster father", "former foster dad", "former foster parent", "former foster brother", "former foster sister",
  "grandma", "grandpa", "first cousin", "second cousin", "grandchild", "adoptive grandchild", "adopted grandchild", "adopted grandson",
  "adoptive grandson", "adoptive granddaughter", "adopted granddaughter", "adoptive uncle", "adoptive aunt", "half brother", "half sister",
  "biological mother", "biological father", "biological mom", "biological dad", "adoptive first cousin", "adoptive second cousin", "adoptive cousin",
  "foster sibling", "former foster sibling", "bio mom", "bio dad", 
];

pub const FORMAL_ROLES: [&'static str; 53] = [
  "intensive care coordinator", "icc", "family partner", "fp", "in home therapist", "in-home therapist", "iht",
  "in home behavioral therapist", "in-home behavioral therapist", "ihbt", "therapeutic mentor", "tm", "ot", "occupational therapist",
  "psychiatrist", "outpatient therapist", "opt", "guidance counselor", "school social worker", "social worker", "dcf worker",
  "guardian ad litem", "asentria worker", "mentor", "therapist", "behavioral therapist", "parole officer", "primary care physician",
  "pcp", "therapeutic training and support", "therapeutic training & support", "tt&s", "tt and s", "dmh worker", "clinician",
  "teacher", "special education teacher", "school guidance counselor", "lifeset worker", "lifeset mentor", "bcba", "yapm", "young adult peer mentor",
  "case manager", "dds case manager", "dmh case manager", "dcf worker", "dcf social worker", "behavior monitor", "bm", "hospital case manager",
  "mci family partner", "mobile crisis intervention family partner"
];

pub const INDIRECT_ROLES: [&'static str; 25] = [
  "director", "clinical director", "principal", "assistant director", "assistant clinical director", "director of special education",
  "clinical supervisor", "director of social and emotional learning", "assistant director of special education",
  "crisis support worker", "crisis clinician", "crisis response clinician", "mci worker", "mci clinician", "mobile crisis intervention",
  "emergency services clinician", "emergency services", "crisis assessment clinician",
  "mobile crisis intervention clinician", "mobile crisis intervention worker", "crisis support clinician", "crisis response worker",
  "mobile crisis clinician", "intake coordinator", "supervisor"
];

pub const DEFAULT_NOTE_TEMPLATES: [(&'static str, &'static str); 1] = [
  (
    "Care Plan Meeting",
    "\
      This is a template for (---u---) for the client (---c---). \
    ",
  ),
];