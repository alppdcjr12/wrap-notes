use std::{fmt, io, thread, time};
use ansi_term::Colour::{Black, Red, Blue};

// print errors
pub fn print_err(s: String) {
  print!("{}", Black.on(Red).paint(s));
}
macro_rules! println_err {
    () => (print_err("\n"));
    ($fmt:expr) => (print_err(concat!($fmt, "\n").to_string()));
    ($fmt:expr, $($arg:tt)*) => (print_err!(concat!($fmt, "\n"), $($arg)*));
}
// print instructions
pub fn print_inst(s: String) {
  print!("{}", Black.on(Blue).paint(s));
}
macro_rules! print_inst {
    ($($arg:tt)*) => (print_inst(format!("{}", format_args!($($arg)*))));
}
macro_rules! println_inst {
    () => (print_inst("\n"));
    ($fmt:expr) => (print_inst(concat!($fmt, "\n").to_string()));
    ($fmt:expr, $($arg:tt)*) => (print_inst!(concat!($fmt, "\n"), $($arg)*));
}

pub trait BlankIterator {
  fn fill_in_category(&self) -> String;
  fn alpha_index(&self) -> String;
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum InternalDocumentFillIn {
  InternalReferralForm,
  ComprehensiveAssessment,
  ChildAndAdolescentNeedsAndStrengths,
  ChildAndAdolescentNeedsAndStrengthsReassessment,
  StrengthsNeedsAndCulturalDiscovery,
  IndividualCarePlan,
  SignedReleases,
  UnsignedReleases,
  TransitionSummary,
  FinancialAgreement,
  InformedConsent,
  TechnologyPlan,
  TelehealthTechnologySafetyPlan,
  TelehealthInformedConsent,
  CANSVirtualGatewayConsentForm,
  WFIConsentForm,
  OtherInternalDocument,
}

use InternalDocumentFillIn::{
  InternalReferralForm,
  ComprehensiveAssessment,
  ChildAndAdolescentNeedsAndStrengths,
  ChildAndAdolescentNeedsAndStrengthsReassessment,
  StrengthsNeedsAndCulturalDiscovery,
  IndividualCarePlan,
  SignedReleases,
  UnsignedReleases,
  TransitionSummary,
  FinancialAgreement,
  InformedConsent,
  TechnologyPlan,
  TelehealthTechnologySafetyPlan,
  TelehealthInformedConsent,
  CANSVirtualGatewayConsentForm,
  WFIConsentForm,
  OtherInternalDocument,
};

impl InternalDocumentFillIn {
  pub fn is_custom(&self) -> bool {
    match self {
      OtherInternalDocument => true,
      _ => false,
    }
  }
  pub fn selected_display(&self) -> String {
    if !self.is_custom() {
      self.to_string()
    } else {
      println_inst!("Enter text for the selected blank.");
      'get_string: loop {
        let mut input = String::new();
        let input_result = io::stdin().read_line(&mut input);
        match input_result {
          Err(_) => {
            println_err!("Invalid entry.");
            thread::sleep(time::Duration::from_secs(2));
            continue;
          }
          Ok(_) => {
            break loop {
              println_inst!("Confirm custom fill-in: '{}'? ( Y / N )", &input);
              let mut confirm_input = String::new();
              let confirm_input_result = io::stdin().read_line(&mut confirm_input);
              match confirm_input_result {
                Err(_) => {
                  println_err!("Invalid entry.");
                  thread::sleep(time::Duration::from_secs(2));
                  continue;
                }
                Ok(_) => {
                  match &confirm_input.trim().to_ascii_lowercase()[..] {
                    "yes" | "y" => break input.trim().to_string(),
                    "n" | "no" => continue 'get_string,
                    _ => {
                      println_err!("Invalid entry.");
                      thread::sleep(time::Duration::from_secs(2));
                      continue;
                    }
                  }
                }
              }
            };
          }
        }
      }
    }
  }
  pub fn iterator_of_blanks() -> Box<dyn Iterator<Item = InternalDocumentFillIn>> {
    Box::new([
      InternalReferralForm,
      ComprehensiveAssessment,
      ChildAndAdolescentNeedsAndStrengths,
      ChildAndAdolescentNeedsAndStrengthsReassessment,
      StrengthsNeedsAndCulturalDiscovery,
      IndividualCarePlan,
      SignedReleases,
      UnsignedReleases,
      TransitionSummary,
      FinancialAgreement,
      InformedConsent,
      TechnologyPlan,
      TelehealthTechnologySafetyPlan,
      TelehealthInformedConsent,
      CANSVirtualGatewayConsentForm,
      WFIConsentForm,
      OtherInternalDocument,
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
      InternalReferralForm => "Wraparound referral form",
      ComprehensiveAssessment => "Comprehensive Assessment",
      ChildAndAdolescentNeedsAndStrengths => "CANS",
      ChildAndAdolescentNeedsAndStrengthsReassessment => "CANS reassessment",
      StrengthsNeedsAndCulturalDiscovery => "Strengths, Needs and Cultural Discovery",
      IndividualCarePlan => "Individual Care Plan",
      SignedReleases => "signed Release of Information forms for youth",
      UnsignedReleases => "unsigned Release of Information forms for youth",
      TransitionSummary => "transition summary",
      FinancialAgreement => "Financial Agreement",
      InformedConsent => "Informed Consent",
      TechnologyPlan => "Technology Plan",
      TelehealthTechnologySafetyPlan => "Telehealth Technology Safety Plan",
      TelehealthInformedConsent => "Telehealth Informed Consent Form",
      CANSVirtualGatewayConsentForm => "CANS Virtual Gateway Consent Form",
      WFIConsentForm => "WFI Consent Form",
      // keep last for the sake of organization
      OtherInternalDocument => "other internal document",
    };
    write!(f, "{}", display_string)
  }
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum ExternalDocumentFillIn {
  NeuropsychologicalAssessment,
  SchoolAssessment,
  IndividualEducationPlan,
  ExternalReferralForm,
  DischargeForm,
  FunctionalBehavioralAssessment,
  OtherExternalDocument,
}

use ExternalDocumentFillIn::{
  NeuropsychologicalAssessment,
  SchoolAssessment,
  IndividualEducationPlan,
  ExternalReferralForm,
  DischargeForm,
  FunctionalBehavioralAssessment,
  OtherExternalDocument,
};

impl ExternalDocumentFillIn {
  pub fn is_custom(&self) -> bool {
    match self {
      OtherExternalDocument => true,
      _ => false,
    }
  }
  pub fn selected_display(&self) -> String {
    if !self.is_custom() {
      self.to_string()
    } else {
      println_inst!("Enter text for the selected blank.");
      'get_string: loop {
        let mut input = String::new();
        let input_result = io::stdin().read_line(&mut input);
        match input_result {
          Err(_) => {
            println_err!("Invalid entry.");
            thread::sleep(time::Duration::from_secs(2));
            continue;
          }
          Ok(_) => {
            break loop {
              println_inst!("Confirm custom fill-in: '{}'? ( Y / N )", &input);
              let mut confirm_input = String::new();
              let confirm_input_result = io::stdin().read_line(&mut confirm_input);
              match confirm_input_result {
                Err(_) => {
                  println_err!("Invalid entry.");
                  thread::sleep(time::Duration::from_secs(2));
                  continue;
                }
                Ok(_) => {
                  match &confirm_input.trim().to_ascii_lowercase()[..] {
                    "yes" | "y" => break input.trim().to_string(),
                    "n" | "no" => continue 'get_string,
                    _ => {
                      println_err!("Invalid entry.");
                      thread::sleep(time::Duration::from_secs(2));
                      continue;
                    }
                  }
                }
              }
            };
          }
        }
      }
    }
  }
  pub fn iterator_of_blanks() -> Box<dyn Iterator<Item = ExternalDocumentFillIn>> {
    Box::new([
      NeuropsychologicalAssessment,
      SchoolAssessment,
      IndividualEducationPlan,
      ExternalReferralForm,
      DischargeForm,
      FunctionalBehavioralAssessment,
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
      ExternalReferralForm => "referral form",
      DischargeForm => "discharge form",
      FunctionalBehavioralAssessment => "Functional Behavioral Assessment",
      // keep last for organization
      OtherExternalDocument => "other external document",
    };
    write!(f, "{}", display_string)
  }
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum InternalMeetingFillIn {
  IntakeMeeting,
  AssessmentMeeting,
  SncdMeeting,
  HomeVisitMeeting,
  AgendaPrepMeeting,
  CarePlanMeeting,
  DebriefMeeting,
  CheckInMeeting,
  TransitionMeeting,
  ConcurrentReviewOfInsuranceAuthorization,
  OtherInternalMeeting
}

use InternalMeetingFillIn::{
  IntakeMeeting,
  AssessmentMeeting,
  SncdMeeting,
  HomeVisitMeeting,
  AgendaPrepMeeting,
  CarePlanMeeting,
  DebriefMeeting,
  CheckInMeeting,
  TransitionMeeting,
  ConcurrentReviewOfInsuranceAuthorization,
  OtherInternalMeeting
};

impl InternalMeetingFillIn {
  pub fn is_custom(&self) -> bool {
    match self {
      OtherInternalMeeting => true,
      _ => false,
    }
  }
  pub fn selected_display(&self) -> String {
    if !self.is_custom() {
      self.to_string()
    } else {
      println_inst!("Enter text for the selected blank.");
      'get_string: loop {
        let mut input = String::new();
        let input_result = io::stdin().read_line(&mut input);
        match input_result {
          Err(_) => {
            println_err!("Invalid entry.");
            thread::sleep(time::Duration::from_secs(2));
            continue;
          }
          Ok(_) => {
            break loop {
              println_inst!("Confirm custom fill-in: '{}'? ( Y / N )", &input);
              let mut confirm_input = String::new();
              let confirm_input_result = io::stdin().read_line(&mut confirm_input);
              match confirm_input_result {
                Err(_) => {
                  println_err!("Invalid entry.");
                  thread::sleep(time::Duration::from_secs(2));
                  continue;
                }
                Ok(_) => {
                  match &confirm_input.trim().to_ascii_lowercase()[..] {
                    "yes" | "y" => break input.trim().to_string(),
                    "n" | "no" => continue 'get_string,
                    _ => {
                      println_err!("Invalid entry.");
                      thread::sleep(time::Duration::from_secs(2));
                      continue;
                    }
                  }
                }
              }
            };
          }
        }
      }
    }
  }
  pub fn iterator_of_blanks() -> Box<dyn Iterator<Item = InternalMeetingFillIn>> {
    Box::new([
      IntakeMeeting,
      AssessmentMeeting,
      SncdMeeting,
      HomeVisitMeeting,
      AgendaPrepMeeting,
      CarePlanMeeting,
      DebriefMeeting,
      CheckInMeeting,
      TransitionMeeting,
      ConcurrentReviewOfInsuranceAuthorization,
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
      SncdMeeting => "SNCD",
      HomeVisitMeeting => "home visit",
      AgendaPrepMeeting => "agenda prep",
      CarePlanMeeting => "Care Plan Meeting",
      DebriefMeeting => "debrief",
      CheckInMeeting => "check in",
      TransitionMeeting => "transition meeting",
      ConcurrentReviewOfInsuranceAuthorization => "concurrent review of insurance authorization",
      // keep last for organization
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
  RecommendationMeeting,
  CourtAppointment,
  DischargeMeeting,
  ReentryMeeting,
  OtherExternalMeeting
}

use ExternalMeetingFillIn::{
  IEPMeeting,
  SchoolAssessmentMeeting,
  Consult,
  TreatmentTeamMeeting,
  RecommendationMeeting,
  CourtAppointment,
  DischargeMeeting,
  ReentryMeeting,
  OtherExternalMeeting,
};

impl ExternalMeetingFillIn {
  pub fn is_custom(&self) -> bool {
    match self {
      OtherExternalMeeting => true,
      _ => false,
    }
  }
  pub fn selected_display(&self) -> String {
    if !self.is_custom() {
      self.to_string()
    } else {
      println_inst!("Enter text for the selected blank.");
      'get_string: loop {
        let mut input = String::new();
        let input_result = io::stdin().read_line(&mut input);
        match input_result {
          Err(_) => {
            println_err!("Invalid entry.");
            thread::sleep(time::Duration::from_secs(2));
            continue;
          }
          Ok(_) => {
            break loop {
              println_inst!("Confirm custom fill-in: '{}'? ( Y / N )", &input);
              let mut confirm_input = String::new();
              let confirm_input_result = io::stdin().read_line(&mut confirm_input);
              match confirm_input_result {
                Err(_) => {
                  println_err!("Invalid entry.");
                  thread::sleep(time::Duration::from_secs(2));
                  continue;
                }
                Ok(_) => {
                  match &confirm_input.trim().to_ascii_lowercase()[..] {
                    "yes" | "y" => break input.trim().to_string(),
                    "n" | "no" => continue 'get_string,
                    _ => {
                      println_err!("Invalid entry.");
                      thread::sleep(time::Duration::from_secs(2));
                      continue;
                    }
                  }
                }
              }
            };
          }
        }
      }
    }
  }
  pub fn iterator_of_blanks() -> Box<dyn Iterator<Item = ExternalMeetingFillIn>> {
    Box::new([
      IEPMeeting,
      SchoolAssessmentMeeting,
      Consult,
      TreatmentTeamMeeting,
      RecommendationMeeting,
      CourtAppointment,
      DischargeMeeting,
      ReentryMeeting,
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
      RecommendationMeeting => "recommendation meeting",
      CourtAppointment => "court appointment",
      DischargeMeeting => "discharge meeting",
      ReentryMeeting => "reentry meeting",
      // keep last for organization
      OtherExternalMeeting => "other external meeting",
    };
    write!(f, "{}", display_string)
  }
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum AppearanceFillIn {
  Angry,
  Anxious,
  Busy,
  Communicative,
  Concerned,
  Cooperative,
  Determined,
  Disappointed,
  Disengaged,
  Distracted,
  Engaged,
  Excited,
  Flat,
  Focused,
  Friendly,
  Frustrated,
  Guarded,
  Happy,
  Honest,
  Hyperactive,
  Motivated,
  Open,
  Overwhelmed,
  Positive,
  Receptive,
  Relaxed,
  Responsive,
  Restless,
  Rigid,
  Sad,
  Silly,
  Sleepy,
  Slumped,
  Stressed,
  Tired,
  Tranquil,
  Unkempt,
  Upset,
  OtherAppearance,
}

use AppearanceFillIn::{
  Angry,
  Anxious,
  Busy,
  Communicative,
  Concerned,
  Cooperative,
  Determined,
  Disappointed,
  Disengaged,
  Distracted,
  Engaged,
  Excited,
  Flat,
  Focused,
  Friendly,
  Frustrated,
  Guarded,
  Happy,
  Honest,
  Hyperactive,
  Motivated,
  Open,
  Overwhelmed,
  Positive,
  Receptive,
  Relaxed,
  Responsive,
  Restless,
  Rigid,
  Sad,
  Silly,
  Sleepy,
  Slumped,
  Stressed,
  Tired,
  Tranquil,
  Unkempt,
  Upset,
  OtherAppearance,
};

impl AppearanceFillIn {
  pub fn is_custom(&self) -> bool {
    match self {
      OtherAppearance => true,
      _ => false,
    }
  }
  pub fn selected_display(&self) -> String {
    if !self.is_custom() {
      self.to_string()
    } else {
      println_inst!("Enter text for the selected blank.");
      'get_string: loop {
        let mut input = String::new();
        let input_result = io::stdin().read_line(&mut input);
        match input_result {
          Err(_) => {
            println_err!("Invalid entry.");
            thread::sleep(time::Duration::from_secs(2));
            continue;
          }
          Ok(_) => {
            break loop {
              println_inst!("Confirm custom fill-in: '{}'? ( Y / N )", &input);
              let mut confirm_input = String::new();
              let confirm_input_result = io::stdin().read_line(&mut confirm_input);
              match confirm_input_result {
                Err(_) => {
                  println_err!("Invalid entry.");
                  thread::sleep(time::Duration::from_secs(2));
                  continue;
                }
                Ok(_) => {
                  match &confirm_input.trim().to_ascii_lowercase()[..] {
                    "yes" | "y" => break input.trim().to_string(),
                    "n" | "no" => continue 'get_string,
                    _ => {
                      println_err!("Invalid entry.");
                      thread::sleep(time::Duration::from_secs(2));
                      continue;
                    }
                  }
                }
              }
            };
          }
        }
      }
    }
  }
  pub fn iterator_of_blanks() -> Box<dyn Iterator<Item = AppearanceFillIn>> {
    Box::new([
      Angry,
      Anxious,
      Busy,
      Communicative,
      Concerned,
      Cooperative,
      Determined,
      Disappointed,
      Disengaged,
      Distracted,
      Engaged,
      Excited,
      Flat,
      Focused,
      Friendly,
      Frustrated,
      Guarded,
      Happy,
      Honest,
      Hyperactive,
      Motivated,
      Open,
      Overwhelmed,
      Positive,
      Receptive,
      Relaxed,
      Responsive,
      Restless,
      Rigid,
      Sad,
      Silly,
      Sleepy,
      Slumped,
      Stressed,
      Tired,
      Tranquil,
      Unkempt,
      Upset,
      OtherAppearance,
    ].iter().copied())
  }
}

impl BlankIterator for AppearanceFillIn {
  fn fill_in_category(&self) -> String {
    String::from("appearance/affect")
  }
  fn alpha_index(&self) -> String {
    String::from("apa")
  }
}

impl fmt::Display for AppearanceFillIn {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let display_string = match self {
      Angry => "angry",
      Anxious => "anxious",
      Busy => "busy",
      Communicative => "communicative",
      Concerned => "concerned",
      Cooperative => "cooperative",
      Determined => "determined",
      Disappointed => "disappointed",
      Disengaged => "disengaged",
      Distracted => "distracted",
      Engaged => "engaged",
      Excited => "excited",
      Flat => "flat",
      Focused => "focused",
      Friendly => "friendly",
      Frustrated => "frustrated",
      Guarded => "guarded",
      Happy => "happy",
      Honest => "honest",
      Hyperactive => "hyperactive",
      Motivated => "motivated",
      Open => "open",
      Overwhelmed => "overwhelmed",
      Positive => "positive",
      Receptive => "receptive",
      Relaxed => "relaxed",
      Responsive => "responsive",
      Restless => "restless",
      Rigid => "rigid",
      Sad => "sad",
      Silly => "silly",
      Sleepy => "sleepy",
      Slumped => "slumped",
      Stressed => "stressed",
      Tired => "tired",
      Tranquil => "tranquil",
      Unkempt => "unkempt",
      Upset => "upset",
      OtherAppearance => "other appearance/affect",
    };
    write!(f, "{}", display_string)
  }
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum SupportedParentFillIn {
  AdvocatedFor,
  AffirmedSupport,
  Applauded,
  AskedOpenEndedQuestionsOf,
  CheeredOn,
  CompletedAnExerciseWith,
  CompletedPaperworkWith,
  ElicitedChangeTalkFrom,
  ExplainedParentingConceptsTo,
  ExplainedTheRoleOfProvidersTo,
  GaveFeedbackTo,
  IdentifiedCopingSkillsWith,
  IntroducedConceptsTo,
  MadeRecommendationsTo,
  ProvidedPsychoeducationTo,
  Reflected,
  EmpathizedWith,
  SharedParentingPerspectivesWith,
  SuggestedResourcesTo,
  Supported,
  Validated,
  OtherSupportedParent,
}

use SupportedParentFillIn::{
  AdvocatedFor,
  AffirmedSupport,
  Applauded,
  AskedOpenEndedQuestionsOf,
  CheeredOn,
  CompletedAnExerciseWith,
  CompletedPaperworkWith,
  ElicitedChangeTalkFrom,
  ExplainedParentingConceptsTo,
  ExplainedTheRoleOfProvidersTo,
  GaveFeedbackTo,
  IdentifiedCopingSkillsWith,
  IntroducedConceptsTo,
  MadeRecommendationsTo,
  ProvidedPsychoeducationTo,
  Reflected,
  EmpathizedWith,
  SharedParentingPerspectivesWith,
  SuggestedResourcesTo,
  Supported,
  Validated,
  OtherSupportedParent,
};

impl SupportedParentFillIn {
  pub fn is_custom(&self) -> bool {
    match self {
      OtherSupportedParent => true,
      _ => false,
    }
  }
  pub fn selected_display(&self) -> String {
    if !self.is_custom() {
      self.to_string()
    } else {
      println_inst!("Enter text for the selected blank.");
      'get_string: loop {
        let mut input = String::new();
        let input_result = io::stdin().read_line(&mut input);
        match input_result {
          Err(_) => {
            println_err!("Invalid entry.");
            thread::sleep(time::Duration::from_secs(2));
            continue;
          }
          Ok(_) => {
            break loop {
              println_inst!("Confirm custom fill-in: '{}'? ( Y / N )", &input);
              let mut confirm_input = String::new();
              let confirm_input_result = io::stdin().read_line(&mut confirm_input);
              match confirm_input_result {
                Err(_) => {
                  println_err!("Invalid entry.");
                  thread::sleep(time::Duration::from_secs(2));
                  continue;
                }
                Ok(_) => {
                  match &confirm_input.trim().to_ascii_lowercase()[..] {
                    "yes" | "y" => break input.trim().to_string(),
                    "n" | "no" => continue 'get_string,
                    _ => {
                      println_err!("Invalid entry.");
                      thread::sleep(time::Duration::from_secs(2));
                      continue;
                    }
                  }
                }
              }
            };
          }
        }
      }
    }
  }
  pub fn iterator_of_blanks() -> Box<dyn Iterator<Item = SupportedParentFillIn>> {
    Box::new([
      AdvocatedFor,
      AffirmedSupport,
      Applauded,
      AskedOpenEndedQuestionsOf,
      CheeredOn,
      CompletedAnExerciseWith,
      CompletedPaperworkWith,
      ElicitedChangeTalkFrom,
      ExplainedParentingConceptsTo,
      ExplainedTheRoleOfProvidersTo,
      GaveFeedbackTo,
      IdentifiedCopingSkillsWith,
      IntroducedConceptsTo,
      MadeRecommendationsTo,
      ProvidedPsychoeducationTo,
      Reflected,
      EmpathizedWith,
      SharedParentingPerspectivesWith,
      SuggestedResourcesTo,
      Supported,
      Validated,
      OtherSupportedParent,
    ].iter().copied())
  }
}

impl BlankIterator for SupportedParentFillIn {
  fn fill_in_category(&self) -> String {
    String::from("supported parent")
  }
  fn alpha_index(&self) -> String {
    String::from("sp")
  }
}

impl fmt::Display for SupportedParentFillIn {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let display_string = match self {
      AdvocatedFor => "advocated for",
      AffirmedSupport => "affirmed",
      Applauded => "applauded",
      AskedOpenEndedQuestionsOf => "asked open ended questions of",
      CheeredOn => "cheered on",
      CompletedAnExerciseWith => "completed an exercise with",
      CompletedPaperworkWith => "completed paperwork with",
      ElicitedChangeTalkFrom => "elicited change talk from",
      ExplainedParentingConceptsTo => "explained parenting concepts to",
      ExplainedTheRoleOfProvidersTo => "explained the role of providers to",
      GaveFeedbackTo => "gave feedback to",
      IdentifiedCopingSkillsWith => "identified coping skills with",
      IntroducedConceptsTo => "introduced concepts to",
      MadeRecommendationsTo => "made recommendations to",
      ProvidedPsychoeducationTo => "provided psychoeducation to",
      Reflected => "reflected",
      EmpathizedWith => "empathized with",
      SharedParentingPerspectivesWith => "shared parenting perspectives with",
      SuggestedResourcesTo => "suggested resources to",
      Supported => "supported",
      Validated => "validated",
      OtherSupportedParent => "other supported parent",
    };
    write!(f, "{}", display_string)
  }
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum ParentingSkillFillIn {
  AdvocacyForOnesChild,
  AffectManagement,
  Attunement,
  AwarenessOfOnesEnergyLevels,
  AwarenessOfOnesPushButtons,
  AwarenessOfOnesChildsNeeds,
  Communication,
  CopingSkills,
  CrisisManagement,
  Discipline,
  EmotionalCoregulation,
  EmotionalResponsiveness,
  EmotionalSelfRegulation,
  IdentifyingPushButtons,
  IfThenStatements,
  Ignoring,
  LabellingValidatingAndCoaching,
  LimitSetting,
  Praising,
  PromptingBeforeTransitions,
  ReadingTheirChildsSignals,
  ReinforcingPositiveBehavior,
  SelfCare,
  SelfAdvocacy,
  SelfAwareness,
  TeachingKidsAboutFeelings,
  TimeManagement,
  TraumaSensitivity,
  Validation,
  WorkingOnGoals,
  OtherParentingSkill,
}

use ParentingSkillFillIn::{
  AdvocacyForOnesChild,
  AffectManagement,
  Attunement,
  AwarenessOfOnesEnergyLevels,
  AwarenessOfOnesPushButtons,
  AwarenessOfOnesChildsNeeds,
  Communication,
  CopingSkills,
  CrisisManagement,
  Discipline,
  EmotionalCoregulation,
  EmotionalResponsiveness,
  EmotionalSelfRegulation,
  IdentifyingPushButtons,
  IfThenStatements,
  Ignoring,
  LabellingValidatingAndCoaching,
  LimitSetting,
  Praising,
  PromptingBeforeTransitions,
  ReadingTheirChildsSignals,
  ReinforcingPositiveBehavior,
  SelfCare,
  SelfAdvocacy,
  SelfAwareness,
  TeachingKidsAboutFeelings,
  TimeManagement,
  TraumaSensitivity,
  Validation,
  WorkingOnGoals,
  OtherParentingSkill,
};

impl ParentingSkillFillIn {
  pub fn is_custom(&self) -> bool {
    match self {
      OtherParentingSkill => true,
      _ => false,
    }
  }
  pub fn selected_display(&self) -> String {
    if !self.is_custom() {
      self.to_string()
    } else {
      println_inst!("Enter text for the selected blank.");
      'get_string: loop {
        let mut input = String::new();
        let input_result = io::stdin().read_line(&mut input);
        match input_result {
          Err(_) => {
            println_err!("Invalid entry.");
            thread::sleep(time::Duration::from_secs(2));
            continue;
          }
          Ok(_) => {
            break loop {
              println_inst!("Confirm custom fill-in: '{}'? ( Y / N )", &input);
              let mut confirm_input = String::new();
              let confirm_input_result = io::stdin().read_line(&mut confirm_input);
              match confirm_input_result {
                Err(_) => {
                  println_err!("Invalid entry.");
                  thread::sleep(time::Duration::from_secs(2));
                  continue;
                }
                Ok(_) => {
                  match &confirm_input.trim().to_ascii_lowercase()[..] {
                    "yes" | "y" => break input.trim().to_string(),
                    "n" | "no" => continue 'get_string,
                    _ => {
                      println_err!("Invalid entry.");
                      thread::sleep(time::Duration::from_secs(2));
                      continue;
                    }
                  }
                }
              }
            };
          }
        }
      }
    }
  }
  pub fn iterator_of_blanks() -> Box<dyn Iterator<Item = ParentingSkillFillIn>> {
    Box::new([
      AdvocacyForOnesChild,
      AffectManagement ,
      Attunement,
      AwarenessOfOnesEnergyLevels,
      AwarenessOfOnesPushButtons,
      AwarenessOfOnesChildsNeeds,
      Communication,
      CopingSkills,
      CrisisManagement,
      Discipline,
      EmotionalCoregulation,
      EmotionalResponsiveness,
      EmotionalSelfRegulation,
      IdentifyingPushButtons,
      IfThenStatements,
      Ignoring,
      LabellingValidatingAndCoaching,
      LimitSetting,
      Praising,
      PromptingBeforeTransitions,
      ReadingTheirChildsSignals,
      ReinforcingPositiveBehavior,
      SelfCare,
      SelfAdvocacy,
      SelfAwareness,
      TeachingKidsAboutFeelings,
      TimeManagement,
      TraumaSensitivity,
      Validation,
      WorkingOnGoals,
      OtherParentingSkill,
    ].iter().copied())
  }
}

impl BlankIterator for ParentingSkillFillIn {
  fn fill_in_category(&self) -> String {
    String::from("parenting skill")
  }
  fn alpha_index(&self) -> String {
    String::from("ps")
  }
}

impl fmt::Display for ParentingSkillFillIn {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let display_string = match self {
      AdvocacyForOnesChild => "advocacy for one's child",
      AffectManagement => "affect management",
      Attunement => "attunement",
      AwarenessOfOnesEnergyLevels => "awareness of one's energy levels",
      AwarenessOfOnesPushButtons => "awareness of one's push buttons",
      AwarenessOfOnesChildsNeeds => "awareness of one's child's needs",
      Communication => "communication",
      CopingSkills => "coping skills",
      CrisisManagement => "crisis management",
      Discipline => "discipline",
      EmotionalCoregulation => "emotional coregulation",
      EmotionalResponsiveness => "emotional responsiveness",
      EmotionalSelfRegulation => "emotional self-regulation",
      IdentifyingPushButtons => "identifying push buttons",
      IfThenStatements => "if/then statements",
      Ignoring => "ignoring",
      LabellingValidatingAndCoaching => "labeling, validating and coaching",
      LimitSetting => "limit setting",
      Praising => "praising",
      PromptingBeforeTransitions => "prompting before transitions",
      ReadingTheirChildsSignals => "reading their child's signals",
      ReinforcingPositiveBehavior => "reinforcing positive behaviors",
      SelfCare => "self care",
      SelfAdvocacy => "self-advocacy",
      SelfAwareness => "self-awareness",
      TeachingKidsAboutFeelings => "teaching kids about feelings",
      TimeManagement => "time management",
      TraumaSensitivity => "trauma sensitivity",
      Validation => "validation",
      WorkingOnGoals => "working on goals",
      OtherParentingSkill => "other parenting skill",
    };
    write!(f, "{}", display_string)
  }
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum CarePlanningTopicFillIn {
  AvailabilityOfServices,
  BrainstormedOptions,
  CarePlanMeetingTasks,
  CarePlanTeamMembers,
  FamilyCulture,
  FamilyStrengths,
  FamilyVision,
  GroundRules,
  MeetingObjectives,
  OpeningServices,
  PotentialFutureGoals,
  PotentialObstaclesToGoals,
  PrograssOnActionSteps,
  ProsAndConsOfTreatmentOptions,
  RecentIncidents,
  Scheduling,
  StrategiesForCarePlanning,
  StrategiesForWorkingOnYouthGoals,
  StrengthsOfTheYouth,
  TeamMission,
  TheImplementationOfActionSteps,
  TheMostRecentCarePlanMeeting,
  UpcomingMeetings,
  YouthGoals,
  OtherCarePlanningTopic,
}

use CarePlanningTopicFillIn::{
  AvailabilityOfServices,
  BrainstormedOptions,
  CarePlanMeetingTasks,
  CarePlanTeamMembers,
  FamilyCulture,
  FamilyStrengths,
  FamilyVision,
  GroundRules,
  MeetingObjectives,
  OpeningServices,
  PotentialFutureGoals,
  PotentialObstaclesToGoals,
  PrograssOnActionSteps,
  ProsAndConsOfTreatmentOptions,
  RecentIncidents,
  Scheduling,
  StrategiesForCarePlanning,
  StrategiesForWorkingOnYouthGoals,
  StrengthsOfTheYouth,
  TeamMission,
  TheImplementationOfActionSteps,
  TheMostRecentCarePlanMeeting,
  UpcomingMeetings,
  YouthGoals,
  OtherCarePlanningTopic,
};

impl CarePlanningTopicFillIn {
  pub fn is_custom(&self) -> bool {
    match self {
      OtherCarePlanningTopic => true,
      _ => false,
    }
  }
  pub fn selected_display(&self) -> String {
    if !self.is_custom() {
      self.to_string()
    } else {
      println_inst!("Enter text for the selected blank.");
      'get_string: loop {
        let mut input = String::new();
        let input_result = io::stdin().read_line(&mut input);
        match input_result {
          Err(_) => {
            println_err!("Invalid entry.");
            thread::sleep(time::Duration::from_secs(2));
            continue;
          }
          Ok(_) => {
            break loop {
              println_inst!("Confirm custom fill-in: '{}'? ( Y / N )", &input);
              let mut confirm_input = String::new();
              let confirm_input_result = io::stdin().read_line(&mut confirm_input);
              match confirm_input_result {
                Err(_) => {
                  println_err!("Invalid entry.");
                  thread::sleep(time::Duration::from_secs(2));
                  continue;
                }
                Ok(_) => {
                  match &confirm_input.trim().to_ascii_lowercase()[..] {
                    "yes" | "y" => break input.trim().to_string(),
                    "n" | "no" => continue 'get_string,
                    _ => {
                      println_err!("Invalid entry.");
                      thread::sleep(time::Duration::from_secs(2));
                      continue;
                    }
                  }
                }
              }
            };
          }
        }
      }
    }
  }
  pub fn iterator_of_blanks() -> Box<dyn Iterator<Item = CarePlanningTopicFillIn>> {
    Box::new([
      AvailabilityOfServices,
      BrainstormedOptions,
      CarePlanMeetingTasks,
      CarePlanTeamMembers,
      FamilyCulture,
      FamilyStrengths,
      FamilyVision,
      GroundRules,
      MeetingObjectives,
      OpeningServices,
      PotentialFutureGoals,
      PotentialObstaclesToGoals,
      PrograssOnActionSteps,
      ProsAndConsOfTreatmentOptions,
      RecentIncidents,
      Scheduling,
      StrategiesForCarePlanning,
      StrategiesForWorkingOnYouthGoals,
      StrengthsOfTheYouth,
      TeamMission,
      TheImplementationOfActionSteps,
      TheMostRecentCarePlanMeeting,
      UpcomingMeetings,
      YouthGoals,
      OtherCarePlanningTopic,
    ].iter().copied())
  }
}

impl BlankIterator for CarePlanningTopicFillIn {
  fn fill_in_category(&self) -> String {
    String::from("Care Planning topic")
  }
  fn alpha_index(&self) -> String {
    String::from("cpt")
  }
}

impl fmt::Display for CarePlanningTopicFillIn {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let display_string = match self {
      AvailabilityOfServices => "availability of services",
      BrainstormedOptions => "brainstormed options",
      CarePlanMeetingTasks => "care plan meeting tasks",
      CarePlanTeamMembers => "care plan team members",
      FamilyCulture => "family culture",
      FamilyStrengths => "family strengths",
      FamilyVision => "family vision",
      GroundRules => "ground rules",
      MeetingObjectives => "meeting objectives",
      OpeningServices => "opening services",
      PotentialFutureGoals => "potential future goals",
      PotentialObstaclesToGoals => "potential obstacles to goals",
      PrograssOnActionSteps => "progress on action steps",
      ProsAndConsOfTreatmentOptions => "pros and cons of treatment options",
      RecentIncidents => "recent incidents",
      Scheduling => "scheduling",
      StrategiesForCarePlanning => "strategies for care planning",
      StrategiesForWorkingOnYouthGoals => "strategies for working on youth goals",
      StrengthsOfTheYouth => "strengths of the youth",
      TeamMission => "team mission",
      TheImplementationOfActionSteps => "the implementation of action steps",
      TheMostRecentCarePlanMeeting => "the most recent Care Plan Meeting",
      UpcomingMeetings => "upcoming meetings",
      YouthGoals => "youth goals",
      OtherCarePlanningTopic => "other Care Planning topic",
    };
    write!(f, "{}", display_string)
  }
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum YouthTopicFillIn {
  AcademicParticipation,
  AcademicPerformance,
  ADLSkills,
  Affect,
  Aggression,
  Anxiety,
  AttachmentStyle,
  BehaviorsInTheCommunity,
  CarePlanActionSteps,
  CaregiverInvolvement,
  ChallengingBehaviors,
  CommunicationYouthTopic,
  CommunityActivities,
  CopingSkillsYouthTopic,
  CulturalIdentity,
  Defiance,
  Depression,
  DevelopmentalChanges,
  DisorderedEating,
  EducationalGoals,
  EmotionalNeeds,
  EngagementWithServices,
  ExecutiveFunctioning,
  ExperienceOfPsychosis,
  FamilyCultureYouthTopic,
  FamilyDynamics,
  FamilyEnvironment,
  FrustrationTolerance,
  FunctioningAtHome,
  FunctioningInSchool,
  Hobbies,
  HomicidalIdeation,
  HousingSituation,
  IdentityDevelopment,
  InappropriateLanguage,
  IndependentLivingSkills,
  Interests,
  InterpersonalSkills,
  MedicalNeeds,
  MedicationAdherence,
  MentalHealth,
  NaturalSupports,
  Needs,
  OverallFunctioning,
  PhysicalNeeds,
  DevelopmentalNeeds,
  RoleModels,
  SchoolAttendance,
  SelfEsteem,
  SelfRegulation,
  Sleep,
  SocialAnxiety,
  SocialFunctioning,
  SpiritualOrReligiousStrengths,
  SubstanceUse,
  SuicidalIdeation,
  Symptoms,
  TransitionToIndependentLiving,
  TraumaResponse,
  TreatmentGoals,
  TreatmentSessions,
  Treatment,
  Values,
  TransitionToAdulthood,
  VocationalSkills,
  WeightGain,
  WeightLoss,
  OtherYouthTopic,
}

use YouthTopicFillIn::{
  AcademicParticipation,
  AcademicPerformance,
  ADLSkills,
  Affect,
  Aggression,
  Anxiety,
  AttachmentStyle,
  BehaviorsInTheCommunity,
  CarePlanActionSteps,
  CaregiverInvolvement,
  ChallengingBehaviors,
  CommunicationYouthTopic,
  CommunityActivities,
  CopingSkillsYouthTopic,
  CulturalIdentity,
  Defiance,
  Depression,
  DevelopmentalChanges,
  DisorderedEating,
  EducationalGoals,
  EmotionalNeeds,
  EngagementWithServices,
  ExecutiveFunctioning,
  ExperienceOfPsychosis,
  FamilyCultureYouthTopic,
  FamilyDynamics,
  FamilyEnvironment,
  FrustrationTolerance,
  FunctioningAtHome,
  FunctioningInSchool,
  Hobbies,
  HomicidalIdeation,
  HousingSituation,
  IdentityDevelopment,
  InappropriateLanguage,
  IndependentLivingSkills,
  Interests,
  InterpersonalSkills,
  MedicalNeeds,
  MedicationAdherence,
  MentalHealth,
  NaturalSupports,
  Needs,
  OverallFunctioning,
  PhysicalNeeds,
  DevelopmentalNeeds,
  RoleModels,
  SchoolAttendance,
  SelfEsteem,
  SelfRegulation,
  Sleep,
  SocialAnxiety,
  SocialFunctioning,
  SpiritualOrReligiousStrengths,
  SubstanceUse,
  SuicidalIdeation,
  Symptoms,
  TransitionToIndependentLiving,
  TraumaResponse,
  TreatmentGoals,
  TreatmentSessions,
  Treatment,
  Values,
  TransitionToAdulthood,
  VocationalSkills,
  WeightGain,
  WeightLoss,
  OtherYouthTopic,
};

impl YouthTopicFillIn {
  pub fn is_custom(&self) -> bool {
    match self {
      OtherYouthTopic => true,
      _ => false,
    }
  }
  pub fn selected_display(&self) -> String {
    if !self.is_custom() {
      self.to_string()
    } else {
      println_inst!("Enter text for the selected blank.");
      'get_string: loop {
        let mut input = String::new();
        let input_result = io::stdin().read_line(&mut input);
        match input_result {
          Err(_) => {
            println_err!("Invalid entry.");
            thread::sleep(time::Duration::from_secs(2));
            continue;
          }
          Ok(_) => {
            break loop {
              println_inst!("Confirm custom fill-in: '{}'? ( Y / N )", &input);
              let mut confirm_input = String::new();
              let confirm_input_result = io::stdin().read_line(&mut confirm_input);
              match confirm_input_result {
                Err(_) => {
                  println_err!("Invalid entry.");
                  thread::sleep(time::Duration::from_secs(2));
                  continue;
                }
                Ok(_) => {
                  match &confirm_input.trim().to_ascii_lowercase()[..] {
                    "yes" | "y" => break input.trim().to_string(),
                    "n" | "no" => continue 'get_string,
                    _ => {
                      println_err!("Invalid entry.");
                      thread::sleep(time::Duration::from_secs(2));
                      continue;
                    }
                  }
                }
              }
            };
          }
        }
      }
    }
  }
  pub fn iterator_of_blanks() -> Box<dyn Iterator<Item = YouthTopicFillIn>> {
    Box::new([
      AcademicParticipation,
      AcademicPerformance,
      ADLSkills,
      Affect,
      Aggression,
      Anxiety,
      AttachmentStyle,
      BehaviorsInTheCommunity,
      CarePlanActionSteps,
      CaregiverInvolvement,
      ChallengingBehaviors,
      CommunicationYouthTopic,
      CommunityActivities,
      CopingSkillsYouthTopic,
      CulturalIdentity,
      Defiance,
      Depression,
      DevelopmentalChanges,
      DisorderedEating,
      EducationalGoals,
      EmotionalNeeds,
      EngagementWithServices,
      ExecutiveFunctioning,
      ExperienceOfPsychosis,
      FamilyCultureYouthTopic,
      FamilyDynamics,
      FamilyEnvironment,
      FrustrationTolerance,
      FunctioningAtHome,
      FunctioningInSchool,
      Hobbies,
      HomicidalIdeation,
      HousingSituation,
      IdentityDevelopment,
      InappropriateLanguage,
      IndependentLivingSkills,
      Interests,
      InterpersonalSkills,
      MedicalNeeds,
      MedicationAdherence,
      MentalHealth,
      NaturalSupports,
      Needs,
      OverallFunctioning,
      PhysicalNeeds,
      DevelopmentalNeeds,
      RoleModels,
      SchoolAttendance,
      SelfEsteem,
      SelfRegulation,
      Sleep,
      SocialAnxiety,
      SocialFunctioning,
      SpiritualOrReligiousStrengths,
      SubstanceUse,
      SuicidalIdeation,
      Symptoms,
      TransitionToIndependentLiving,
      TraumaResponse,
      TreatmentGoals,
      TreatmentSessions,
      Treatment,
      Values,
      TransitionToAdulthood,
      VocationalSkills,
      WeightGain,
      WeightLoss,
      OtherYouthTopic,
    ].iter().copied())
  }
}

impl BlankIterator for YouthTopicFillIn {
  fn fill_in_category(&self) -> String {
    String::from("youth mental health topic")
  }
  fn alpha_index(&self) -> String {
    String::from("ymt")
  }
}

impl fmt::Display for YouthTopicFillIn {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let display_string = match self {
      AcademicParticipation => "academic participation",
      AcademicPerformance => "academic performance",
      ADLSkills => "ADL skills",
      Affect => "affect",
      Aggression => "aggression",
      Anxiety => "anxiety",
      AttachmentStyle => "attachment style",
      BehaviorsInTheCommunity => "behaviors in the community",
      CarePlanActionSteps => "Care Plan action steps",
      CaregiverInvolvement => "caregiver involvemnt",
      ChallengingBehaviors => "challenging behaviors",
      CommunicationYouthTopic => "communication",
      CommunityActivities => "community activities",
      CopingSkillsYouthTopic => "coping skills",
      CulturalIdentity => "cultural identity",
      Defiance => "defiance",
      Depression => "depression",
      DevelopmentalChanges => "developmental changes",
      DisorderedEating => "disordered eating",
      EducationalGoals => "educational goals",
      EmotionalNeeds => "emotional needs",
      EngagementWithServices => "engagement with services",
      ExecutiveFunctioning => "executive functioning",
      ExperienceOfPsychosis => "experience of psychosis",
      FamilyCultureYouthTopic => "family culture",
      FamilyDynamics => "family dynamics",
      FamilyEnvironment => "family environment",
      FrustrationTolerance => "frustration tolerance",
      FunctioningAtHome => "functioning at home",
      FunctioningInSchool => "functioning in school",
      Hobbies => "hobbies",
      HomicidalIdeation => "homicidal ideation",
      HousingSituation => "housing situation",
      IdentityDevelopment => "identity development",
      InappropriateLanguage => "inappropriate language",
      IndependentLivingSkills => "independent living skills",
      Interests => "interests",
      InterpersonalSkills => "interpersonal skills",
      MedicalNeeds => "medical needs",
      MedicationAdherence => "medication adherence",
      MentalHealth => "mental health",
      NaturalSupports => "natural supports",
      Needs => "needs",
      OverallFunctioning => "overall functioning",
      PhysicalNeeds => "physical needs",
      DevelopmentalNeeds => "developmental needs",
      RoleModels => "role models",
      SchoolAttendance => "school attendance",
      SelfEsteem => "self-esteem",
      SelfRegulation => "self-regulation",
      Sleep => "sleep",
      SocialAnxiety => "social anxiety",
      SocialFunctioning => "social functioning",
      SpiritualOrReligiousStrengths => "spiritual or religious strengths",
      SubstanceUse => "substance use",
      SuicidalIdeation => "suicidal ideation",
      Symptoms => "symptoms",
      TransitionToIndependentLiving => "transition to independent living",
      TraumaResponse => "trauma responses",
      TreatmentGoals => "treatment goals",
      TreatmentSessions => "treatment sessions",
      Treatment => "treatment",
      Values => "values",
      TransitionToAdulthood => "transition to adulthood",
      VocationalSkills => "vocational skills",
      WeightGain => "weight gain",
      WeightLoss => "weight loss",
      OtherYouthTopic => "other youth mental health topic",
    };
    write!(f, "{}", display_string)
  }
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum ContactMethodFillIn {
  Phone,
  Email,
  Text,
  Voicemail,
  Fax,
  Mail,
  ConferenceCall,
  OtherContactMethod,
}

use ContactMethodFillIn::{
  Phone,
  Email,
  Text,
  Voicemail,
  Fax,
  Mail,
  ConferenceCall,
  OtherContactMethod,
};

impl ContactMethodFillIn {
  pub fn is_custom(&self) -> bool {
    match self {
      OtherContactMethod => true,
      _ => false,
    }
  }
  pub fn selected_display(&self) -> String {
    if !self.is_custom() {
      self.to_string()
    } else {
      println_inst!("Enter text for the selected blank.");
      'get_string: loop {
        let mut input = String::new();
        let input_result = io::stdin().read_line(&mut input);
        match input_result {
          Err(_) => {
            println_err!("Invalid entry.");
            thread::sleep(time::Duration::from_secs(2));
            continue;
          }
          Ok(_) => {
            break loop {
              println_inst!("Confirm custom fill-in: '{}'? ( Y / N )", &input);
              let mut confirm_input = String::new();
              let confirm_input_result = io::stdin().read_line(&mut confirm_input);
              match confirm_input_result {
                Err(_) => {
                  println_err!("Invalid entry.");
                  thread::sleep(time::Duration::from_secs(2));
                  continue;
                }
                Ok(_) => {
                  match &confirm_input.trim().to_ascii_lowercase()[..] {
                    "yes" | "y" => break input.trim().to_string(),
                    "n" | "no" => continue 'get_string,
                    _ => {
                      println_err!("Invalid entry.");
                      thread::sleep(time::Duration::from_secs(2));
                      continue;
                    }
                  }
                }
              }
            };
          }
        }
      }
    }
  }
  pub fn iterator_of_blanks() -> Box<dyn Iterator<Item = ContactMethodFillIn>> {
    Box::new([
      Phone,
      Email,
      Text,
      Voicemail,
      Fax,
      Mail,
      ConferenceCall,
      OtherContactMethod,
    ].iter().copied())
  }
}

impl BlankIterator for ContactMethodFillIn {
  fn fill_in_category(&self) -> String {
    String::from("contact method")
  }
  fn alpha_index(&self) -> String {
    String::from("cm")
  }
}

impl fmt::Display for ContactMethodFillIn {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let display_string = match self {
      Phone => "phone",
      Email => "email",
      Text => "text",
      Voicemail => "voicemail",
      Fax => "fax",
      Mail => "mail",
      ConferenceCall => "conference call",
      OtherContactMethod => "other contact method",
    };
    write!(f, "{}", display_string)
  }
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum ContactPurposeFillIn {
  AskForInformationAboutServices,
  CollectSignedReleaseOfInformationForms,
  CompleteAnAssessment,
  CompleteIntakePaperwork,
  CreateSafetyPlan,
  ExtendAnInvitationToTheYouthsMextCarePlanMeeting,
  LearnAboutPotentialReferrals,
  OfferAvailabilityForScheduling,
  RequestVerbalConsentToExchangePersonalHealthInformationForYouth,
  ReviewPaperwork,
  ReviewTheICP,
  ReviewTheSNCD,
  UpdateThemOnCommunicationWithProviders,
  UpdateThemOnCommunicationWithTheYouthsFamily,
  UpdateSafetyPlan,
  OtherContactPurpose,
}

use ContactPurposeFillIn::{
  AskForInformationAboutServices,
  CollectSignedReleaseOfInformationForms,
  CompleteAnAssessment,
  CompleteIntakePaperwork,
  CreateSafetyPlan,
  ExtendAnInvitationToTheYouthsMextCarePlanMeeting,
  LearnAboutPotentialReferrals,
  OfferAvailabilityForScheduling,
  RequestVerbalConsentToExchangePersonalHealthInformationForYouth,
  ReviewPaperwork,
  ReviewTheICP,
  ReviewTheSNCD,
  UpdateThemOnCommunicationWithProviders,
  UpdateThemOnCommunicationWithTheYouthsFamily,
  UpdateSafetyPlan,
  OtherContactPurpose,
};

impl ContactPurposeFillIn {
  pub fn is_custom(&self) -> bool {
    match self {
      OtherContactPurpose => true,
      _ => false,
    }
  }
  pub fn selected_display(&self) -> String {
    if !self.is_custom() {
      self.to_string()
    } else {
      println_inst!("Enter text for the selected blank.");
      'get_string: loop {
        let mut input = String::new();
        let input_result = io::stdin().read_line(&mut input);
        match input_result {
          Err(_) => {
            println_err!("Invalid entry.");
            thread::sleep(time::Duration::from_secs(2));
            continue;
          }
          Ok(_) => {
            break loop {
              println_inst!("Confirm custom fill-in: '{}'? ( Y / N )", &input);
              let mut confirm_input = String::new();
              let confirm_input_result = io::stdin().read_line(&mut confirm_input);
              match confirm_input_result {
                Err(_) => {
                  println_err!("Invalid entry.");
                  thread::sleep(time::Duration::from_secs(2));
                  continue;
                }
                Ok(_) => {
                  match &confirm_input.trim().to_ascii_lowercase()[..] {
                    "yes" | "y" => break input.trim().to_string(),
                    "n" | "no" => continue 'get_string,
                    _ => {
                      println_err!("Invalid entry.");
                      thread::sleep(time::Duration::from_secs(2));
                      continue;
                    }
                  }
                }
              }
            };
          }
        }
      }
    }
  }
  pub fn iterator_of_blanks() -> Box<dyn Iterator<Item = ContactPurposeFillIn>> {
    Box::new([
      AskForInformationAboutServices,
      CollectSignedReleaseOfInformationForms,
      CompleteAnAssessment,
      CompleteIntakePaperwork,
      CreateSafetyPlan,
      ExtendAnInvitationToTheYouthsMextCarePlanMeeting,
      LearnAboutPotentialReferrals,
      OfferAvailabilityForScheduling,
      RequestVerbalConsentToExchangePersonalHealthInformationForYouth,
      ReviewPaperwork,
      ReviewTheICP,
      ReviewTheSNCD,
      UpdateThemOnCommunicationWithProviders,
      UpdateThemOnCommunicationWithTheYouthsFamily,
      UpdateSafetyPlan,
      OtherContactPurpose,
    ].iter().copied())
  }
}

impl BlankIterator for ContactPurposeFillIn {
  fn fill_in_category(&self) -> String {
    String::from("contact purpose")
  }
  fn alpha_index(&self) -> String {
    String::from("cp")
  }
}

impl fmt::Display for ContactPurposeFillIn {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let display_string = match self {
      AskForInformationAboutServices => "ask for information about services",
      CollectSignedReleaseOfInformationForms => "collect signed Release of Information Forms",
      CompleteAnAssessment => "complete an assessment",
      CompleteIntakePaperwork => "complete intake paperwork",
      CreateSafetyPlan => "create a safety plan",
      ExtendAnInvitationToTheYouthsMextCarePlanMeeting => "extend an invitation to the youth's next Care Plan meeting",
      LearnAboutPotentialReferrals => "learn about potential referrals",
      OfferAvailabilityForScheduling => "offer availability for scheduling",
      RequestVerbalConsentToExchangePersonalHealthInformationForYouth => "request verbal consent to exchange Personal Health Information for youth",
      ReviewPaperwork => "review paperwork",
      ReviewTheICP => "review the ICP",
      ReviewTheSNCD => "review the SNCD",
      UpdateThemOnCommunicationWithProviders => "update them on communication with providers",
      UpdateThemOnCommunicationWithTheYouthsFamily => "update them on communication with the youth's family",
      UpdateSafetyPlan => "update the safety plan for youth",
      OtherContactPurpose => "other contact purpose",
    };
    write!(f, "{}", display_string)
  }
}
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum FulfilledContactPurposeFillIn {
  AskedForInformationAboutServices,
  CollectedSignedReleaseOfInformationForms,
  CompletedAnAssessment,
  CompletedIntakePaperwork,
  CreatedSafetyPlan,
  ExtendedAnInvitationToTheYouthsMextCarePlanMeeting,
  LearnedAboutPotentialReferrals,
  OfferedAvailabilityForScheduling,
  RequestedVerbalConsentToExchangePersonalHealthInformationForYouth,
  ReviewedPaperwork,
  ReviewedTheICP,
  ReviewedTheSNCD,
  UpdatedThemOnCommunicationWithProviders,
  UpdatedThemOnCommunicationWithTheYouthsFamily,
  UpdatedSafetyPlan,
  OtherFulfilledContactPurpose,
}

use FulfilledContactPurposeFillIn::{
  AskedForInformationAboutServices,
  CollectedSignedReleaseOfInformationForms,
  CompletedAnAssessment,
  CompletedIntakePaperwork,
  CreatedSafetyPlan,
  ExtendedAnInvitationToTheYouthsMextCarePlanMeeting,
  LearnedAboutPotentialReferrals,
  OfferedAvailabilityForScheduling,
  RequestedVerbalConsentToExchangePersonalHealthInformationForYouth,
  ReviewedPaperwork,
  ReviewedTheICP,
  ReviewedTheSNCD,
  UpdatedThemOnCommunicationWithProviders,
  UpdatedThemOnCommunicationWithTheYouthsFamily,
  UpdatedSafetyPlan,
  OtherFulfilledContactPurpose,
};

impl FulfilledContactPurposeFillIn {
  pub fn is_custom(&self) -> bool {
    match self {
      OtherFulfilledContactPurpose => true,
      _ => false,
    }
  }
  pub fn selected_display(&self) -> String {
    if !self.is_custom() {
      self.to_string()
    } else {
      println_inst!("Enter text for the selected blank.");
      'get_string: loop {
        let mut input = String::new();
        let input_result = io::stdin().read_line(&mut input);
        match input_result {
          Err(_) => {
            println_err!("Invalid entry.");
            thread::sleep(time::Duration::from_secs(2));
            continue;
          }
          Ok(_) => {
            break loop {
              println_inst!("Confirm custom fill-in: '{}'? ( Y / N )", &input);
              let mut confirm_input = String::new();
              let confirm_input_result = io::stdin().read_line(&mut confirm_input);
              match confirm_input_result {
                Err(_) => {
                  println_err!("Invalid entry.");
                  thread::sleep(time::Duration::from_secs(2));
                  continue;
                }
                Ok(_) => {
                  match &confirm_input.trim().to_ascii_lowercase()[..] {
                    "yes" | "y" => break input.trim().to_string(),
                    "n" | "no" => continue 'get_string,
                    _ => {
                      println_err!("Invalid entry.");
                      thread::sleep(time::Duration::from_secs(2));
                      continue;
                    }
                  }
                }
              }
            };
          }
        }
      }
    }
  }
  pub fn iterator_of_blanks() -> Box<dyn Iterator<Item = FulfilledContactPurposeFillIn>> {
    Box::new([
      AskedForInformationAboutServices,
      CollectedSignedReleaseOfInformationForms,
      CompletedAnAssessment,
      CompletedIntakePaperwork,
      CreatedSafetyPlan,
      ExtendedAnInvitationToTheYouthsMextCarePlanMeeting,
      LearnedAboutPotentialReferrals,
      OfferedAvailabilityForScheduling,
      RequestedVerbalConsentToExchangePersonalHealthInformationForYouth,
      ReviewedPaperwork,
      ReviewedTheICP,
      ReviewedTheSNCD,
      UpdatedThemOnCommunicationWithProviders,
      UpdatedThemOnCommunicationWithTheYouthsFamily,
      UpdatedSafetyPlan,
      OtherFulfilledContactPurpose,
    ].iter().copied())
  }
}

impl BlankIterator for FulfilledContactPurposeFillIn {
  fn fill_in_category(&self) -> String {
    String::from("contact purpose")
  }
  fn alpha_index(&self) -> String {
    String::from("cp")
  }
}

impl fmt::Display for FulfilledContactPurposeFillIn {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let display_string = match self {
      AskedForInformationAboutServices => "asked for information about services",
      CollectedSignedReleaseOfInformationForms => "collected signed Release of Information Forms",
      CompletedAnAssessment => "completed an assessment",
      CompletedIntakePaperwork => "completed intake paperwork",
      CreatedSafetyPlan => "created a safety plan",
      ExtendedAnInvitationToTheYouthsMextCarePlanMeeting => "extended an invitation to the youth's next Care Plan meeting",
      LearnedAboutPotentialReferrals => "learned about potential referrals",
      OfferedAvailabilityForScheduling => "offered availability for scheduling",
      RequestedVerbalConsentToExchangePersonalHealthInformationForYouth => "requested verbal consent to exchange Personal Health Information for youth",
      ReviewedPaperwork => "reviewed paperwork",
      ReviewedTheICP => "reviewed the ICP",
      ReviewedTheSNCD => "reviewed the SNCD",
      UpdatedThemOnCommunicationWithProviders => "updated them on communication with providers",
      UpdatedThemOnCommunicationWithTheYouthsFamily => "updated them on communication with the youth's family",
      UpdatedSafetyPlan => "updated the safety plan for youth",
      OtherFulfilledContactPurpose => "other contact purpose (past tense)",
    };
    write!(f, "{}", display_string)
  }
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum ServiceFillIn {
  InHomeTherapy,
  InHomeBehavioralTherapy,
  AppliedBehavioralAnalysis,
  OutpatientTherapy,
  Respite,
  SkillsTraining,
  PersonalCareAttendant,
  DDSServices,
  DMHServices,
  DCFServices,
  OtherService,
}

use ServiceFillIn::{
  InHomeTherapy,
  InHomeBehavioralTherapy,
  AppliedBehavioralAnalysis,
  OutpatientTherapy,
  Respite,
  SkillsTraining,
  PersonalCareAttendant,
  DDSServices,
  DMHServices,
  DCFServices,
  OtherService,
};

impl ServiceFillIn {
  pub fn is_custom(&self) -> bool {
    match self {
      OtherService => true,
      _ => false,
    }
  }
  pub fn selected_display(&self) -> String {
    if !self.is_custom() {
      self.to_string()
    } else {
      println_inst!("Enter text for the selected blank.");
      'get_string: loop {
        let mut input = String::new();
        let input_result = io::stdin().read_line(&mut input);
        match input_result {
          Err(_) => {
            println_err!("Invalid entry.");
            thread::sleep(time::Duration::from_secs(2));
            continue;
          }
          Ok(_) => {
            break loop {
              println_inst!("Confirm custom fill-in: '{}'? ( Y / N )", &input);
              let mut confirm_input = String::new();
              let confirm_input_result = io::stdin().read_line(&mut confirm_input);
              match confirm_input_result {
                Err(_) => {
                  println_err!("Invalid entry.");
                  thread::sleep(time::Duration::from_secs(2));
                  continue;
                }
                Ok(_) => {
                  match &confirm_input.trim().to_ascii_lowercase()[..] {
                    "yes" | "y" => break input.trim().to_string(),
                    "n" | "no" => continue 'get_string,
                    _ => {
                      println_err!("Invalid entry.");
                      thread::sleep(time::Duration::from_secs(2));
                      continue;
                    }
                  }
                }
              }
            };
          }
        }
      }
    }
  }
  pub fn iterator_of_blanks() -> Box<dyn Iterator<Item = ServiceFillIn>> {
    Box::new([
      InHomeTherapy,
      InHomeBehavioralTherapy,
      AppliedBehavioralAnalysis,
      OutpatientTherapy,
      Respite,
      SkillsTraining,
      PersonalCareAttendant,
      DDSServices,
      DMHServices,
      DCFServices,
      OtherService,
    ].iter().copied())
  }
}

impl BlankIterator for ServiceFillIn {
  fn fill_in_category(&self) -> String {
    String::from("service")
  }
  fn alpha_index(&self) -> String {
    String::from("se")
  }
}

impl fmt::Display for ServiceFillIn {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let display_string = match self {
      InHomeTherapy => "In-Home Therapy",
      InHomeBehavioralTherapy => "In-Home Behavioral Therapy",
      AppliedBehavioralAnalysis => "Applied Behavioral Analysis",
      OutpatientTherapy => "Outpatient Therapy",
      Respite => "Respite",
      SkillsTraining => "skills training",
      PersonalCareAttendant => "Personal Care Attendant",
      DDSServices => "DDS services",
      DMHServices => "DMH services",
      DCFServices => "DMH services",
      OtherService => "other service",
    };
    write!(f, "{}", display_string)
  }
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum MeetingMethodFillIn {
  Zoom,
  InPerson,
  Webex,
  GoogleMeets,
  ConferenceCallMeeting,
  PhoneMeeting,
  OtherMeetingMethod,
}

use MeetingMethodFillIn::{
  Zoom,
  InPerson,
  Webex,
  GoogleMeets,
  ConferenceCallMeeting,
  PhoneMeeting,
  OtherMeetingMethod,
};

impl MeetingMethodFillIn {
  pub fn is_custom(&self) -> bool {
    match self {
      OtherMeetingMethod => true,
      _ => false,
    }
  }
  pub fn selected_display(&self) -> String {
    if !self.is_custom() {
      self.to_string()
    } else {
      println_inst!("Enter text for the selected blank.");
      'get_string: loop {
        let mut input = String::new();
        let input_result = io::stdin().read_line(&mut input);
        match input_result {
          Err(_) => {
            println_err!("Invalid entry.");
            thread::sleep(time::Duration::from_secs(2));
            continue;
          }
          Ok(_) => {
            break loop {
              println_inst!("Confirm custom fill-in: '{}'? ( Y / N )", &input);
              let mut confirm_input = String::new();
              let confirm_input_result = io::stdin().read_line(&mut confirm_input);
              match confirm_input_result {
                Err(_) => {
                  println_err!("Invalid entry.");
                  thread::sleep(time::Duration::from_secs(2));
                  continue;
                }
                Ok(_) => {
                  match &confirm_input.trim().to_ascii_lowercase()[..] {
                    "yes" | "y" => break input.trim().to_string(),
                    "n" | "no" => continue 'get_string,
                    _ => {
                      println_err!("Invalid entry.");
                      thread::sleep(time::Duration::from_secs(2));
                      continue;
                    }
                  }
                }
              }
            };
          }
        }
      }
    }
  }
  pub fn iterator_of_blanks() -> Box<dyn Iterator<Item = MeetingMethodFillIn>> {
    Box::new([
      Zoom,
      InPerson,
      Webex,
      GoogleMeets,
      ConferenceCallMeeting,
      PhoneMeeting,
      OtherMeetingMethod,
    ].iter().copied())
  }
}

impl BlankIterator for MeetingMethodFillIn {
  fn fill_in_category(&self) -> String {
    String::from("meeting method")
  }
  fn alpha_index(&self) -> String {
    String::from("mm")
  }
}

impl fmt::Display for MeetingMethodFillIn {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let display_string = match self {
      Zoom => "via Zoom",
      InPerson => "in person",
      Webex => "via Webex",
      GoogleMeets => "via Google Meets",
      ConferenceCallMeeting => "via conference call",
      PhoneMeeting => "via phone",
      OtherMeetingMethod => "other meeting method",
    };
    write!(f, "{}", display_string)
  }
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum SignatureMethodFillIn {
  ElectronicallySigned,
  SignedByHand,
  AgreedToReceiveSignAndReturnToRiversideStaff,
  OtherSignatureMethod,
}

use SignatureMethodFillIn::{
  ElectronicallySigned,
  SignedByHand,
  AgreedToReceiveSignAndReturnToRiversideStaff,
  OtherSignatureMethod,
};

impl SignatureMethodFillIn {
  pub fn is_custom(&self) -> bool {
    match self {
      OtherSignatureMethod => true,
      _ => false,
    }
  }
  pub fn selected_display(&self) -> String {
    if !self.is_custom() {
      self.to_string()
    } else {
      println_inst!("Enter text for the selected blank.");
      'get_string: loop {
        let mut input = String::new();
        let input_result = io::stdin().read_line(&mut input);
        match input_result {
          Err(_) => {
            println_err!("Invalid entry.");
            thread::sleep(time::Duration::from_secs(2));
            continue;
          }
          Ok(_) => {
            break loop {
              println_inst!("Confirm custom fill-in: '{}'? ( Y / N )", &input);
              let mut confirm_input = String::new();
              let confirm_input_result = io::stdin().read_line(&mut confirm_input);
              match confirm_input_result {
                Err(_) => {
                  println_err!("Invalid entry.");
                  thread::sleep(time::Duration::from_secs(2));
                  continue;
                }
                Ok(_) => {
                  match &confirm_input.trim().to_ascii_lowercase()[..] {
                    "yes" | "y" => break input.trim().to_string(),
                    "n" | "no" => continue 'get_string,
                    _ => {
                      println_err!("Invalid entry.");
                      thread::sleep(time::Duration::from_secs(2));
                      continue;
                    }
                  }
                }
              }
            };
          }
        }
      }
    }
  }
  pub fn iterator_of_blanks() -> Box<dyn Iterator<Item = SignatureMethodFillIn>> {
    Box::new([
      ElectronicallySigned,
      SignedByHand,
      AgreedToReceiveSignAndReturnToRiversideStaff,
      OtherSignatureMethod,
    ].iter().copied())
  }
}

impl BlankIterator for SignatureMethodFillIn {
  fn fill_in_category(&self) -> String {
    String::from("signature method")
  }
  fn alpha_index(&self) -> String {
    String::from("sm")
  }
}

impl fmt::Display for SignatureMethodFillIn {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let display_string = match self {
      ElectronicallySigned => "electronically signed",
      SignedByHand => "signed by hand",
      AgreedToReceiveSignAndReturnToRiversideStaff => "agreed to receive, sign and return to Riverside staff",
      OtherSignatureMethod => "other signature method",
    };
    write!(f, "{}", display_string)
  }
}