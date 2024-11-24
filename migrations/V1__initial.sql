-- AcademicYear definition
CREATE TABLE "AcademicYear" (
	"AcademicYear" TEXT,
	CONSTRAINT years_pk PRIMARY KEY ("AcademicYear")
);

-- StudentInfo definition
CREATE TABLE "StudentInfo" (
	"ID"	INTEGER,
	FirstName TEXT NOT NULL,
	LastName TEXT NOT NULL,
	PRIMARY KEY("ID")
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
    rowid INTEGER,
    Alpha INTEGER NOT NULL,
    Red INTEGER NOT NULL,
    Green INTEGER NOT NULL,
    Blue INTEGER NOT NULL,
	PRIMARY KEY (rowid)
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
	CONSTRAINT marks_fill_FK FOREIGN KEY (Fill) REFERENCES "FillColour"(rowid),
	CONSTRAINT FK_marks_student_info FOREIGN KEY (ID) REFERENCES "StudentInfo"(ID),
	CONSTRAINT marks_modules_FK FOREIGN KEY (Module) REFERENCES "Module"(Code)
);

-- "Result" definition
CREATE TABLE "Result" (
	ID INTEGER NOT NULL,
	AcademicYear TEXT NOT NULL,
	"Plan" TEXT NOT NULL,
	YearOfStudy INTEGER NOT NULL,
	AutumnCredits INTEGER,
	AutumnMean REAL,
	SpringCredits INTEGER,
	SpringMean REAL,
	YearCredits INTEGER,
	YearMean REAL,
	Progression INTEGER,
	Remarks TEXT,
	CONSTRAINT results_student_info_FK FOREIGN KEY (ID) REFERENCES StudentInfo(ID),
	CONSTRAINT Result_AcademicYear_FK FOREIGN KEY (AcademicYear) REFERENCES AcademicYear(AcademicYear)
);
