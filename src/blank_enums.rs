use std::fmt;

pub trait BlankIterator {
  fn fill_in_category(&self) -> String;
  fn display_fill_in(&self) -> String {
    format!("{}", &self)
  }
  fn alpha_index(&self) -> String;
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
  OtherInternalDocument,
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

impl InternalDocumentFillIn {
  pub fn iterator_of_blanks() -> Box<dyn Iterator<Item = InternalDocumentFillIn>> {
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
}

impl BlankIterator for InternalDocumentFillIn {
  fn fill_in_category(&self) -> String {
    String::from("internal document")
  }
  fn alpha_index(&self) -> String {
    String::from("rd")
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
  OtherExternalDocument,
}

use ExternalDocumentFillIn::{
  NeuropsychologicalAssessment,
  IndividualEducationPlan,
  OtherExternalDocument,
};

impl ExternalDocumentFillIn {
  pub fn iterator_of_blanks() -> Box<dyn Iterator<Item = ExternalDocumentFillIn>> {
    Box::new([
      NeuropsychologicalAssessment,
      IndividualEducationPlan,
      OtherExternalDocument
    ].iter().copied())
  }
}

impl BlankIterator for ExternalDocumentFillIn {
  fn fill_in_category(&self) -> String {
    String::from("external document")
  }
  fn alpha_index(&self) -> String {
    String::from("ed")
  }
}

impl fmt::Display for ExternalDocumentFillIn {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let display_string = match self {
      NeuropsychologicalAssessment => "neuropsychological assessment",
      SchoolAssessment => "school assessment",
      IndividualEducationPlan => "IEP",
      OtherExternalDocument => "other exxternal document",
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
  OtherInternalMeeting
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
  OtherInternalMeeting
};

impl InternalMeetingFillIn {
  pub fn iterator_of_blanks() -> Box<dyn Iterator<Item = InternalMeetingFillIn>> {
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
      OtherInternalMeeting
    ].iter().copied())
  }
}

impl BlankIterator for InternalMeetingFillIn {
  fn fill_in_category(&self) -> String {
    String::from("Wraparound meeting title")
  }
  fn alpha_index(&self) -> String {
    String::from("rm")
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
      OtherInternalMeeting => "other internal meeting",
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
  OtherExternalMeeting
}

use ExternalMeetingFillIn::{
  IEPMeeting,
  SchoolAssessmentMeeting,
  Consult,
  TreatmentTeamMeeting,
  OtherExternalMeeting,
};

impl ExternalMeetingFillIn {
  pub fn iterator_of_blanks() -> Box<dyn Iterator<Item = ExternalMeetingFillIn>> {
    Box::new([
      IEPMeeting,
      SchoolAssessmentMeeting,
      Consult,
      TreatmentTeamMeeting,
      OtherExternalMeeting,
    ].iter().copied())
  }
}

impl BlankIterator for ExternalMeetingFillIn {
  fn fill_in_category(&self) -> String {
    String::from("external meeting title")
  }
  fn alpha_index(&self) -> String {
    String::from("em")
  }
}

impl fmt::Display for ExternalMeetingFillIn {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let display_string = match self {
      IEPMeeting => "IEP meeting",
      SchoolAssessmentMeeting => "school assessment meeting",
      Consult => "consult",
      TreatmentTeamMeeting => "treatment team meeting",
      OtherExternalMeeting => "other external meeting",
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

use ActionFillIn::{
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

impl ActionFillIn {
  pub fn iterator_of_blanks() -> Box<dyn Iterator<Item = ActionFillIn>> {
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
}

impl BlankIterator for ActionFillIn {
  fn fill_in_category(&self) -> String {
    String::from("general action")
  }
  fn alpha_index(&self) -> String {
    String::from("a")
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

impl PhraseFillIn {
  pub fn iterator_of_blanks() -> Box<dyn Iterator<Item = PhraseFillIn>> {
    Box::new([
      AllTeamMembersPresentAtMeeting,
      AllTeamMembers,
    ].iter().copied())
  }
}

impl BlankIterator for PhraseFillIn {
  fn fill_in_category(&self) -> String {
    String::from("other phrase")
  }
  fn alpha_index(&self) -> String {
    String::from("p")
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
