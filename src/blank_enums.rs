use std::fmt;

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
};

impl AppearanceFillIn {
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
  ExplainedTheRoleOfProviders,
  GaveFeedbackTo,
  IdentifiedCopingSkills,
  IntroducedConceptsTo,
  MadeRecommendationsTo,
  ProvidedPsychoeducationTo,
  Reflected,
  EmpathizedWith,
  SharedParentingPerspectives,
  SuggestedResourcesTo,
  Supported,
  Validated,
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
};

impl SupportedParentFillIn {
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
    };
    write!(f, "{}", display_string)
  }
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum ParentingSkillsFillIn {
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
}

use ParentingSkillsFillIn::{
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
};

impl ParentingSkillsFillIn {
  pub fn iterator_of_blanks() -> Box<dyn Iterator<Item = ParentingSkillsFillIn>> {
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
    ].iter().copied())
  }
}

impl BlankIterator for ParentingSkillsFillIn {
  fn fill_in_category(&self) -> String {
    String::from("parenting skills")
  }
  fn alpha_index(&self) -> String {
    String::from("ps")
  }
}

impl fmt::Display for ParentingSkillsFillIn {
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
};

impl CarePlanningTopicFillIn {
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
  Communication,
  CommunityActivities,
  CopingSkills,
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
  FamilyCulture,
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
  Communication,
  CommunityActivities,
  CopingSkills,
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
  FamilyCulture,
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
};

impl YouthTopicFillIn {
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
      Communication,
      CommunityActivities,
      CopingSkills,
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
      FamilyCulture,
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
      Communication => "communication",
      CommunityActivities => "community activities",
      CopingSkills => "coping skills",
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
      FamilyCulture => "family culture",
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
}

use ContactMethodFillIn::{
  Phone,
  Email,
  Text,
  Voicemail,
  Fax,
  Mail,
  ConferenceCall,
};

impl ContactMethodFillIn {
  pub fn iterator_of_blanks() -> Box<dyn Iterator<Item = ContactMethodFillIn>> {
    Box::new([
      Phone,
      Email,
      Text,
      Voicemail,
      Fax,
      Mail,
      ConferenceCall,
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

impl fmt::Display for ContactPurposeFillIn {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let display_string = match self {
      Phone => "phone",
      Email => "email",
      Text => "text",
      Voicemail => "voicemail",
      Fax => "fax",
      Mail => "mail",
      ConferenceCall => "conference call",
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
  Custom,
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
  Custom,
};

impl ContactPurposeFillIn {
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
      Custom,
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
      Custom => "custom",
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
  DCFServices,
};

impl ServiceFillIn {
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
      DCFServices,
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
  ConferenceCall,
  Phone,
}

use MeetingMethodFillIn::{
  Zoom,
  InPerson,
  Webex,
  GoogleMeets,
  ConferenceCall,
  Phone,
};

impl MeetingMethodFillIn {
  pub fn iterator_of_blanks() -> Box<dyn Iterator<Item = MeetingMethodFillIn>> {
    Box::new([
      Zoom,
      InPerson,
      Webex,
      GoogleMeets,
      ConferenceCall,
      Phone,
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
      ConferenceCall => "via conference call",
      Phone => "via phone",
    };
    write!(f, "{}", display_string)
  }
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum SignedMethodFillIn {
  ElectronicallySigned,
  SignedByHand,
  AgreedToReceiveSignAndReturnToRiversideStaff,
}

use SignedMethodFillIn::{
  ElectronicallySigned,
  SignedByHand,
  AgreedToReceiveSignAndReturnToRiversideStaff,
};

impl SignedMethodFillIn {
  pub fn iterator_of_blanks() -> Box<dyn Iterator<Item = SignedMethodFillIn>> {
    Box::new([
      ElectronicallySigned,
      SignedByHand,
      AgreedToReceiveSignAndReturnToRiversideStaff,
    ].iter().copied())
  }
}

impl BlankIterator for SignedMethodFillIn {
  fn fill_in_category(&self) -> String {
    String::from("signature method")
  }
  fn alpha_index(&self) -> String {
    String::from("sm")
  }
}

impl fmt::Display for SignedMethodFillIn {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let display_string = match self {
      ElectronicallySigned => "electronically signed",
      SignedByHand => "signed by hand",
      AgreedToReceiveSignAndReturnToRiversideStaff => "agreed to receive, sign and return to Riverside staff",
    };
    write!(f, "{}", display_string)
  }
}
