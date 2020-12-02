trait BlankIterator {
  pub fn iterator() -> Box<dyn Iterator>;
  pub fn fill_in_category(&self) -> String;
  pub fn display_fill_in(&self) -> String {
    format!("{}", self)
  }
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum InternalDocumentFillIn {
  ReferralForm,
  TelehealthConsent,
  TechnologyPlan,
  FinancialAgreement,
  InformedConsent,
  ComprehensiveAssessment,
  ChildAndAdolescentNeedsAndStrengths,
  StrengthsNeedsAndCulturalDiscovery,
  IndividualCarePlan,
  TransitionSummary,
}

use InternalDocumentFillIn::{
  ReferralForm,
  TelehealthConsent,
  TechnologyPlan,
  FinancialAgreement,
  InformedConsent,
  ComprehensiveAssessment,
  ChildAndAdolescentNeedsAndStrengths,
  StrengthsNeedsAndCulturalDiscovery,
  IndividualCarePlan,
  TransitionSummary,
};

impl BlankIterator for InternalDocumentFillIn {
  pub fn iterator() -> Box<dyn Iterator<Item = InternalDocumentFillIn>> {
    Box::new([
      ReferralForm,
      TelehealthConsent,
      TechnologyPlan,
      FinancialAgreement,
      InformedConsent,
      ComprehensiveAssessment,
      ChildAndAdolescentNeedsAndStrengths,
      StrengthsNeedsAndCulturalDiscovery,
      IndividualCarePlan,
      TransitionSummary,
    ].iter().copied())
  }
  pub fn fill_in_category(&self) -> String {
    String::from("internal document")
  }
}

impl fmt::Display for InternalDocumentFillIn {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let display_string = match self {
      ReferralForm => "referral form",
      TelehealthConsent => "Telehealth Consent Form",
      TechnologyPlan => "Technology Plan",
      FinancialAgreement => "Financial Agreement",
      InformedConsent => "Informed Consent",
      ComprehensiveAssessment => "Comprehensive Assessment",
      ChildAndAdolescentNeedsAndStrengths => "CANS",
      StrengthsNeedsAndCulturalDiscovery => "Strengths, Needs and Cultural Discovery",
      IndividualCarePlan => "Individual Care Plan",
      TransitionSummary => "transition summary",
    };
    write!(f, "{}", display_string)
  }
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum ExternalDocumentFillIn {
  NeuropsychologicalAssessment,
  IndividualEducationPlan,
  Other
}

use ExternalDocumentFillIn::{
  NeuropsychologicalAssessment,
  IndividualEducationPlan,
  Other,
};

impl BlankIterator for ExternalDocumentFillIn {
  pub fn iterator() -> Box<dyn Iterator<Item = ExternalDocumentFillIn>> {
    Box::new([
      NeuropsychologicalAssessment,
      IndividualEducationPlan,
      Other
    ].iter().copied())
  }
  pub fn fill_in_category(&self) -> String {
    String::from("external document")
  }
}

impl fmt::Display for ExternalDocumentFillIn {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let display_string = match self {
      NeuropsychologicalAssessment => "neuropsychological assessment",
      SchoolAssessment => "school assessment",
      IndividualEducationPlan => "IEP",
      Other => "other exxternal document",
    };
    write!(f, "{}", display_string)
  }
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum InternalMeetingFillIn {
  IntakeMeeting,
  AssessmentMeeting,
  SNCDMeeting,
  HomeVisitMeeting,
  AgendaPrepMeeting,
  CarePlanMeeting,
  DebriefMeeting,
  CheckInMeeting,
  TransitionMeeting,
  Other
}

use InternalMeetingFillIn::{
  IntakeMeeting,
  AssessmentMeeting,
  SNCDMeeting,
  HomeVisitMeeting,
  AgendaPrepMeeting,
  CarePlanMeeting,
  DebriefMeeting,
  CheckInMeeting,
  TransitionMeeting,
  Other
};

impl BlankIterator for InternalMeetingFillIn {
  pub fn iterator() -> Box<dyn Iterator<Item = InternalMeetingFillIn>> {
    Box::new([
      IntakeMeeting,
      AssessmentMeeting,
      SNCDMeeting,
      HomeVisitMeeting,
      AgendaPrepMeeting,
      CarePlanMeeting,
      DebriefMeeting,
      CheckInMeeting,
      TransitionMeeting,
      Other
    ].iter().copied())
  }
  pub fn fill_in_category(&self) -> String {
    String::from("Wraparound meeting title")
  }
}

impl fmt::Display for InternalMeetingFillIn {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let display_string = match self {
      IntakeMeeting => "intake",
      AssessmentMeeting => "assessment",
      SNCDMeeting => "SNCD",
      HomeVisitMeeting => "home visit",
      AgendaPrepMeeting => "agenda prep",
      CarePlanMeeting => "Care Plan Meeting",
      DebriefMeeting => "debrief",
      CheckInMeeting => "check in",
      TransitionMeeting => "transition meeting",
      Other => "other internal meeting",
    };
    write!(f, "{}", display_string)
  }
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum ExternalMeetingFillIn {
  IEPMeeting,
  SchoolAssessmentMeeting,
  Consult,
  TreatmentTeamMeeting,
}

use ExternalMeetingFillIn {
  IEPMeeting,
  SchoolAssessmentMeeting,
  Consult,
  TreatmentTeamMeeting,
};

impl BlankIterator for ExternalMeetingFillIn {
  pub fn iterator() -> impl Iterator<Item = ExternalMeetingFillIn> {
    Box::new([
      IEPMeeting,
      SchoolAssessmentMeeting,
      Consult,
      TreatmentTeamMeeting,
    ].iter().copied())
  }
  pub fn fill_in_category(&self) -> String {
    String::from("external meeting title")
  }
}

impl fmt::Display for ExternalMeetingFillIn {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let display_string = match self {
      IEPMeeting => "IEP meeting",
      SchoolAssessmentMeeting => "school assessment meeting",
      Consult => "consult",
      TreatmentTeamMeeting => "treatment team meeting",
    };
    write!(f, "{}", display_string)
  }
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum ActionFillIn {
  Called,
  Emailed,
  Texted,
  Elicited,
  Reflected,
  Summarized,
  Scheduled,
  Affirmed,
  Brainstormed,
  Reviewed,
}

use ActionFillIn {
  Called,
  Emailed,
  Texted,
  Elicited,
  Reflected,
  Summarized,
  Scheduled,
  Affirmed,
  Brainstormed,
  Reviewed,
};

impl BlankIterator for ActionFillIn {
  pub fn iterator() -> Box<dyn Iterator<Item = ActionFillIn>> {
    Box::new([
      Called,
      Emailed,
      Texted,
      Elicited,
      Reflected,
      Summarized,
      Scheduled,
      Affirmed,
      Brainstormed,
      Reviewed,
    ].iter().copied())
  }
  pub fn fill_in_category(&self) -> String {
    String::from("general action")
  }
}

impl fmt::Display for ActionFillIn {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let display_string = match self {
      Called => "called",
      Emailed => "emailed",
      Texted => "texted",
      Elicited => "elicited",
      Reflected => "reflected",
      Summarized => "summarized",
      Scheduled => "scheduled",
      Affirmed => "affirmed",
      Brainstormed => "brainstormed",
      Reviewed => "reviewed",
    };
    write!(f, "{}", display_string)
  }
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum PhraseFillIn {
  AllTeamMembersPresentAtMeeting,
  AllTeamMembers,
}

use PhraseFillIn::{
  AllTeamMembersPresentAtMeeting,
  AllTeamMembers,
};

impl BlankIterator for PhraseFillIn {
  pub fn iterator() -> Box<dyn Iterator<Item = PhraseFillIn>> {
    Box::new([
      AllTeamMembersPresentAtMeeting,
      AllTeamMembers,
    ].iter().copied())
  }
  pub fn fill_in_category(&self) -> String {
    String::from("other phrase")
  }
}

impl fmt::Display for PhraseFillIn {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let display_string = match self {
      AllTeamMembersPresentAtMeeting => "all team members for youth present at the meeting",
      AllTeamMembers => "all team members for youth",
    };
    write!(f, "{}", display_string)
  }
}
