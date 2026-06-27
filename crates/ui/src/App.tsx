import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { ColorPaletteGenerator } from "./utils/color";
import { SolveResponse } from "./utils/types";

const PAGES = ["nValues", "persons", "results"] as const;
type Page = (typeof PAGES)[number];

export const App = () => {
  const [nValues, setNValues] = useState({
    nCourses: 6,
    nKLPerCourse: 2,
    nTimeSlots: 6,
  });

  const [persons, setPersons] = useState<string[][]>([]);

  const [currentPage, setCurrentPage] = useState<Page>(PAGES[0]);

  const [result, setResult] = useState<SolveResponse | null>(null);
  const [colorPaletteGenerator, setColorPaletteGenerator] =
    useState<ColorPaletteGenerator>(new ColorPaletteGenerator(0));

  const [highlightedName, setHighlightedName] = useState<string | null>(null);

  const getColor = (absolutePersonIdx: number) => {
    const frac = absolutePersonIdx / (nValues.nCourses * nValues.nKLPerCourse);
    return `hsl(${frac * 360}, 100%, 50%)`;
  };

  useEffect(() => {
    setPersons(
      Array.from({ length: nValues.nCourses }, () =>
        Array.from({ length: nValues.nKLPerCourse }, () => ""),
      ),
    );
  }, [nValues.nCourses, nValues.nKLPerCourse]);

  useEffect(() => {
    if (currentPage !== "results") return;
    solve();
  }, [currentPage]);

  const solve = async () => {
    setResult(null);
    setColorPaletteGenerator(
      new ColorPaletteGenerator(nValues.nCourses * nValues.nKLPerCourse, 10),
    );
    setHighlightedName(null);

    const result: SolveResponse = await invoke("solve", {
      persons,
      nTimeSlots: nValues.nTimeSlots,
    });

    setResult(result);
  };

  const next = () => {
    const index = PAGES.indexOf(currentPage);
    if (index === PAGES.length - 1) return;
    setCurrentPage(PAGES[index + 1]);
  };

  const prev = () => {
    const index = PAGES.indexOf(currentPage);
    if (index === 0) return;
    setCurrentPage(PAGES[index - 1]);
  };

  return (
    <div className="size-full flex justify-center">
      <main className="w-full max-w-400 p-6 pt-10 flex flex-col items-center gap-4">
        {currentPage === "nValues" && (
          <div className="grid grid-rows-3 grid-cols-2 gap-2">
            <label className="font-bold" htmlFor="nCourses">
              Kurse
            </label>
            <input
              type="number"
              id="nCourses"
              value={nValues.nCourses}
              onChange={(e) =>
                setNValues((v) => ({ ...v, nCourses: Number(e.target.value) }))
              }
            />

            <label className="font-bold" htmlFor="nKLPerCourse">
              KL pro Kurs
            </label>
            <input
              type="number"
              id="nKLPerCourse"
              value={nValues.nKLPerCourse}
              onChange={(e) => {
                setNValues((v) => ({
                  ...v,
                  nKLPerCourse: Number(e.target.value),
                }));
              }}
            />

            <label className="font-bold" htmlFor="nTimeSlots">
              Zeitschienen
            </label>
            <input
              type="number"
              id="nTimeSlots"
              value={nValues.nTimeSlots}
              onChange={(e) => {
                setNValues((v) => ({
                  ...v,
                  nTimeSlots: Number(e.target.value),
                }));
              }}
            />
          </div>
        )}

        {currentPage === "persons" && (
          <div className="grid w-full gap-8 grid-cols-3">
            {persons.map((course, courseIdx) => (
              <div key={courseIdx} className="grid gap-4">
                <h2 className="font-bold text-center">Kurs {courseIdx + 1}</h2>
                {course.map((person, coursePersonIdx) => (
                  <div
                    key={coursePersonIdx}
                    className="flex items-center gap-2 justify-center"
                  >
                    <input
                      placeholder={`Kursleiter*in ${coursePersonIdx + 1}`}
                      type="text"
                      value={person}
                      onChange={(e) => {
                        const updatedPersons = [...persons];
                        updatedPersons[courseIdx][coursePersonIdx] =
                          e.target.value;
                        setPersons(updatedPersons);
                      }}
                    />
                  </div>
                ))}
              </div>
            ))}
          </div>
        )}

        {currentPage === "results" &&
          (result !== null ? (
            <>
              <div className="w-full flex gap-2">
                <span className="font-bold">Score:</span>
                <span>{result.total_score}</span>
              </div>

              {highlightedName !== null && (
                <div className="w-full">
                  <button onClick={() => setHighlightedName(null)}>
                    Hervorhebung beenden
                  </button>
                </div>
              )}

              <div
                className="grid w-full *:border *:px-1 *:py-4"
                style={{
                  gridTemplateColumns: `5rem repeat(${result.result[0].length}, 1fr)`,
                }}
              >
                <div className="font-bold bg-foreground text-background border-none">
                  Zeitslot
                </div>
                {result.result[0].map((_, courseIdx) => (
                  <div
                    className="font-bold bg-foreground text-background border-none"
                    key={`header-${courseIdx}`}
                  >
                    Kurs {courseIdx + 1}
                  </div>
                ))}

                {result.result.map((timeSlot, timeSlotIdx) => (
                  <>
                    <div className="font-bold bg-foreground text-background border-none">
                      {timeSlotIdx + 1}
                    </div>
                    {timeSlot.map((persons, courseIdx) => (
                      <div>
                        {persons.map((person, personIdx) => (
                          <div
                            onClick={() => setHighlightedName(person)}
                            className="text-sm font-bold cursor-pointer"
                            key={`${timeSlotIdx}-${courseIdx}-${personIdx}`}
                            style={{
                              color:
                                highlightedName !== null &&
                                highlightedName !== person
                                  ? "#aaaaaa"
                                  : colorPaletteGenerator.getColor(person),
                            }}
                          >
                            {person}
                          </div>
                        ))}
                      </div>
                    ))}
                  </>
                ))}
              </div>
            </>
          ) : (
            <div>Loading</div>
          ))}

        <div className="mt-auto flex w-full justify-between">
          {currentPage !== PAGES[0] ? (
            <button onClick={prev}>Zurück</button>
          ) : (
            <span />
          )}
          {currentPage !== PAGES[PAGES.length - 1] ? (
            <button onClick={next}>Weiter</button>
          ) : result !== null ? (
            <button onClick={solve}>Erneut lösen</button>
          ) : (
            <span />
          )}
        </div>
      </main>
    </div>
  );
};
