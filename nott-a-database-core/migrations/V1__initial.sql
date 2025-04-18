-- AcademicYear definition
CREATE TABLE "AcademicYear" (
	"AcademicYear" TEXT,
	CONSTRAINT years_pk PRIMARY KEY ("AcademicYear")
);

-- StudentInfo definition
CREATE TABLE "StudentInfo" (
	"ID"	INTEGER,
	LastName TEXT NOT NULL,
	FirstName TEXT NOT NULL,
    CareerNo INTEGER,
    Program TEXT,
    ProgramDesc TEXT,
	"Plan" TEXT NOT NULL,
    PlanDesc TEXT,
    INTAKE TEXT,
    QAA TEXT,
    CalcModel TEXT,
    RawMark REAL,
    TruncatedMark REAL,
    FinalMark INTEGER,
    Borderline TEXT,
    Calculation INTEGER,
    DegreeAward TEXT,
    Selected INTEGER,
    ExceptionData TEXT,
    Recommendation TEXT,
    IntakeYear TEXT NOT NULL,
    GraduationYear TEXT,
	PRIMARY KEY("ID"),
	CONSTRAINT FK_intake FOREIGN KEY (IntakeYear) REFERENCES "AcademicYear"(AcademicYear),
	CONSTRAINT FK_graduation_year FOREIGN KEY (GraduationYear) REFERENCES "AcademicYear"(AcademicYear)
);

-- Module definition
CREATE TABLE "Module" (
	Code TEXT NOT NULL,
	Credit INTEGER NOT NULL,
	Name TEXT,
	CONSTRAINT modules_pk PRIMARY KEY (Code)
);

-- Colour Definition
CREATE TABLE "FillColour" (
    rowid INTEGER PRIMARY KEY,
    Alpha INTEGER NOT NULL,
    Red INTEGER NOT NULL,
    Green INTEGER NOT NULL,
    Blue INTEGER NOT NULL
);

-- Mark definition
CREATE TABLE "Mark" (
	ID INTEGER NOT NULL,
	Mark REAL NOT NULL,
    Fill INTEGER,
	Retake1 REAL,
	Retake2 REAL,
	Extra TEXT,
	Module TEXT NOT NULL,
    Status TEXT CHECK ( Status in ("Pass", "CF", "HF", "SF") ) NOT NULL,
	PRIMARY KEY("ID", "Module"),
	CONSTRAINT marks_fill_FK FOREIGN KEY (Fill) REFERENCES "FillColour"(rowid),
	CONSTRAINT FK_marks_student_info FOREIGN KEY (ID) REFERENCES "StudentInfo"(ID),
	CONSTRAINT marks_modules_FK FOREIGN KEY (Module) REFERENCES "Module"(Code)
);

-- "Result" definition
CREATE TABLE "Result" (
	AcademicYear TEXT NOT NULL,
	ID INTEGER NOT NULL,
	YearOfStudy INTEGER NOT NULL,
	AutumnCredits INTEGER,
	AutumnMean REAL,
	SpringCredits INTEGER,
	SpringMean REAL,
	YearCredits INTEGER,
	YearMean REAL,
	Progression TEXT,
	Remarks TEXT,
	PRIMARY KEY("ID", "AcademicYear"),
	CONSTRAINT results_student_info_FK FOREIGN KEY (ID) REFERENCES StudentInfo(ID),
	CONSTRAINT Result_AcademicYear_FK FOREIGN KEY (AcademicYear) REFERENCES AcademicYear(AcademicYear)
);
