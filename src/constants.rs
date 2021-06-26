use ansi_term::Colour;

pub const BG: Colour = Colour::RGB(5, 40, 40);

pub const FAMILY_ROLES: [&'static str; 119] = [
  "family", "parent", "nuclear family", "nuclear family member", "family member", "immediate family", "spouse", "husband", "wife",
  "father", "mother", "step-father", "step father", "stepfather", "step-mother", "step mother", "stepmother", "step-mother", "legal guardian",
  "child", "son", "daughter", "step-son", "step son", "stepson", "step-daughter", "stepdaughter", "step daughter", "sibling", "brother", "sister",
  "extended family", "grandparent", "grandfather", "grandmother", "grandson", "paternal grandparent", "paternal grandfather", "paternal grandmother",
  "maternal grandparent", "maternal grandfather", "maternal grandmother",
  "maternal grandpa", "paternal grandpa", "maternal grandma", "paternal grandma", "uncle", "aunt", "cousin", "nephew", "niece",
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

pub const FORMAL_ROLES: [&'static str; 69] = [
  "intensive care coordinator", "icc", "family partner", "fp", "in home therapist", "in-home therapist", "iht",
  "in home behavioral therapist", "in-home behavioral therapist", "ihbt", "therapeutic mentor", "tm", "ot", "occupational therapist",
  "psychiatrist", "outpatient therapist", "opt", "guidance counselor", "school social worker", "social worker", "dcf worker",
  "guardian ad litem", "asentria worker", "mentor", "therapist", "behavioral therapist", "parole officer", "primary care physician",
  "pcp", "therapeutic training and support", "therapeutic training & support", "tt&s", "tt and s", "dmh worker", "clinician",
  "therapeutic training and support mentor", "therapeutic training & support mentor", "tt&s mentor", "tt and s mentor", 
  "teacher", "special education teacher", "school guidance counselor", "lifeset worker", "lifeset mentor", "bcba", "yapm", "young adult peer mentor",
  "case manager", "dds case manager", "dds case coordinator", "case coordinator", "dmh case manager", "dcf worker", "dcf social worker", "behavior monitor", "bm", "hospital case manager",
  "mci family partner", "mobile crisis intervention family partner", "academic support", "adjustment counselor", "school adjustment counselor",
  "dds service coordinator", "service coordinator", "dds intensive case manager", "intensive case manager",
  "dds intensive flexible family support", "intensive flexible family support", "psychiatric nurse practitioner"
];

pub const INDIRECT_ROLES: [&'static str; 25] = [
  "director", "clinical director", "principal", "assistant director", "assistant clinical director", "director of special education",
  "clinical supervisor", "director of social and emotional learning", "assistant director of special education",
  "crisis support worker", "crisis clinician", "crisis response clinician", "mci worker", "mci clinician", "mobile crisis intervention",
  "emergency services clinician", "emergency services", "crisis assessment clinician",
  "mobile crisis intervention clinician", "mobile crisis intervention worker", "crisis support clinician", "crisis response worker",
  "mobile crisis clinician", "intake coordinator", "supervisor",
];

pub const DEFAULT_NOTE_TEMPLATES: [(&'static str, &'static str); 28] = [
  (
    "Care Plan",
    "\
      (---u---) met with (---cpt---) (---mm---). \
      All team members completed introductions as necessary. \
      (---cu---). \
      The team went over all elements of the agenda including the team mission, \
      family vision, and ground rules. \
      The team went over strengths for (---c---) and (---pb3@5@---) family related to \
      the current goal. \
      Team members provided updates on and discussed (---c---)'s (---yt---). \
      The team discussed updates on (---cpto---) for (---c---). \
      (---cu---). \
      The team brainstormed action steps for (---c---)'s goal of \"(---go---).\" \
      (---u---) scheduled the (---im---) for (---c---) for (---c---).\
    ",
  ),
  (
    "Care Plan",
    "\
      Met with (---co---) in family home in (---cu---) for CPM. \
      (---u---) passed around sign-in sheet. \
      The team reviewed ground rules and team mission. \
      (---u---) reviewed team strengths and family vision. \
      The team brainstormed around the following goal: (---go---). \
      (---u---) assigned tasks to team members. \
      The team reported no major safety concerns.\
    ",
  ),
  (
    "Intake",
    "\
      (---u---) met with (---co---) for an intake for (---c---). \
      (---u---) and (---p---) introduced themselves and discussed \
      (---pc---)'s hopes and expectations for Wraparound using \
      open-ended questions to explore changes the family wants to make. \
      (---cu---). \
      The team elicited (---pc---)'s experience of challenges (---pb1@9@---) and \
      (---c---) have faced, including (---c---)'s (---yt---). \
      The team elicited and reflected the family's strengths, including (---cu---). \
      (---u---) explained the Wraparound process. \
      (---u---) and (---p---) shared information about the limitations and intensive \
      nature of Wraparound services as well as the role of different team members. \
      (---u---) and (---p---) discussed their roles and the structure of Wraparound \
      services. \
      (---u---) reviewed the (---id---) with (---g---). \
      (---u---) elicited verbal consent for the (---id---) and (---g---) (---sm---) \
      the (---id---). \
      (---g---) (---sm---) Release of Information forms permitting (---u---) and (---p---) \
      to exchange (---c---)'s Protected Health Information with (---co---). \
      (---cu---). \
      (---u---) scheduled the (---im---) for (---cu---).\
    ",
  ),
  (
    "Intake",
    "\
      (---u---) met for intake for (---c---) with (---co---) at the family's home in (---cu---). \
      ICC reviewed wraparound process and paperwork. \
      Paperwork included: (---id---). \
      (---pc---) discussed her major needs/concerns for (---c---). \
      (---pc---) reported that Andrew struggles with (---cu---). \
      (---cu---). \
      (---u---) and (---p---) further discussed the Wraparound process with the family \
      and how the process could be tailored to their needs. \
      (---cu---). \
      (---u---) and (---p---) scheduled the (---im---) with the family.\
    ",
  ),
  (
    "Assessment",
    "\
      (---u---) met with (---co---) for the assessment of (---c---). \
      (---pc---) shared updates on (---c---)'s recent (---yt---). \
      (---u---) listened and used open-ended questions to learn more about (---c---)'s \
      experience and the perspectives of (---co---) while also addressing \
      assessment of (---c---)'s recent (---yt---). \
      (---cu---). \
      (---u---) scheduled the (---im---) for (---cu---).\
    ",
  ),
  (
    "Assessment",
    "\
      (---u---) met with (---co---) to complete comprehensive and CANS assessments for (---c---) in the \
      family's home in (---cu---). (---u---) gathered (---c---)'s current needs and challenges. \
      ICC gathered current and past supports and services, family background information, educational information, \
      history of abuse/trauma, medications and medical information, diagnoses, and assessed risk/safety. \
      (---cu---). \
      ICC further assessed (---c---)'s needs and strengths. \
      (---cu---).\
    ",
  ),
  (
    "Agenda Prep",
    "\
      (---u---) met with (---co---) for the Agenda Prep for (---c---)'s next Care Plan Meeting.\
      The team went over all elements of the agenda including the team mission, family vision, \
      and ground rules. \
      The team went over recent updates on (---c---)'s (---yt---). \
      The team discussed potential treatment goals for (---c---) and agreed \
      on addressing (---g---) for the next Care Plan Meeting. \
      (---u---) scheduled the (---im---) for (---cu---).\
    ",
  ),
  (
    "Debrief",
    "\
      (---u---) and (---p---) met with (---pc---) for the debrief of (---c---)'s most \
      recent Care Plan Meeting. \
      The team went over and agreed on action steps to implement, including (---cu---). \
      (---cu---). \
      (---u---) scheduled the (---im---) for (---cu---).\
    ",
  ),
  (
    "Referral",
    "\
      (---u---) sent all required documents via (---cm---) to (---co---) in order to \
      complete a referral for (---c---) for (---s---).\
    ",
  ),
  (
    "Parent Support",
    "\
      (---u---) (---ps---) (---pc---) (---cu---). \
    ",
  ),
  (
    "Parent Appearance",
    "\
      (---co---) appeared (---ap---). \
    ",
  ),
  (
    "Parent Skills",
    "\
      (---co---) demonstrated effective (---ps---). \
    ",
  ),
  (
    "Brainstorm Contribution",
    "\
      (---u---) contributed to the team brainstorm ideas for (---c---)'s \
      Care Plan goal of \"(---go---).\"\
    ",
  ),
  (
    "Collateral Outreach",
    "\
      (---u---) reached out to (---co---) via (---cm---) and (---fcp---). \
    ",
  ),
  (
    "Update From Collateral",
    "\
      (---u---) received a (---cm---) from (---co---) informing (---pu2---) that (---cu---). \
    ",
  ),
  (
    "Discuss Communication",
    "\
      (---u---) called (---co---) to discuss recent communication between (---co---). \
    ",
  ),
  (
    "Invited To Meeting",
    "\
      (---u---) sent Zoom and Outlook invitations for the upcoming (---im---) for (---c---) \
      to (---co---).\
    ",
  ),
  (
    "Failed Contact Attempt",
    "\
      (---u---) reached out to (---co---) via (---cm---) to (---cp---) but was unable to reach (---pb2@2@---). \
    ",
  ),
  (
    "Received Verbal Consent",
    "\
      (---u---) contacted (---g---) via (---cm---) and received verbal consent \
      for (---u---) and Riverside Community Care to exchange (---c---)'s \
      Protected Health Information with (---co---).\
    ",
  ),
  (
    "Received Written Consent",
    "\
      (---u---) received written consent via (---cm---) for (---u---) and Riverside \
      Community Care to exchange (---c---)'s Protected Health Information with (---co---).\
    ",
  ),
  (
    "Documentation",
    "\
      (---u---) reviewed notes and completed daily logs for (---td---). \
    ",
  ),
  (
    "Documentation",
    "\
      (---u---) entered into evolv summaries of treatment events for (---c---) for today, (---td---). \
    ",
  ),
  (
    "Updated Document",
    "\
      (---u---) updated the (---id---) for (---c---) with updates on (---pc3---) (---yt---). \
    ",
  ),
  (
    "Sent Document",
    "\
      (---u---) sent the (---id---) for (---c---) to (---co---) via (---cm---). \
    ",
  ),
  (
    "Sent Cancellation",
    "\
      (---u---) emailed (---co---) to cancel the (---im---) for (---c---). \
    ",
  ),
  (
    "Authorization Requested",
    "\
      (---u---) (---cm---) (---co---) to request a new insurance authorization for (---c---). \
    ",
  ),
  (
    "Authorization Issued",
    "\
      (---u---) received a phone call from (---co---) confirming that a new insurance authorization \
      had been issued for (---c---) for (---cu---) with reference number (---cu---). \
      (---u---) emailed (---co---) to notify (---pb2@7@---).\
    ",
  ),
  (
    "Categorized Emails",
    "\
      (---u---) categorized sent emails for (---c---) into separate folders for record keeping. \
    ",
  ),
];