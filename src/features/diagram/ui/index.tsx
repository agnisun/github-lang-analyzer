import { InputField } from "@entities/input-field";
import { CircularProgress, Container, Flex } from "@chakra-ui/react";
import { LanguageList } from "@entities/language-list";
import { DiagramButtons } from "@entities/diagram-buttons";
import { useAtom } from "jotai";
import { githubLanguagesAtom, isLoadingAtom } from "@shared/model/github";
import { DiagramCanvas } from "@shared/ui/diagram-canvas/diagram-canvas";
import { selectedDiagramAtom } from "@shared/model/diagram";

export const View = () => {
  const [githubLanguages] = useAtom(githubLanguagesAtom);
  const [isLoading] = useAtom(isLoadingAtom);
  const [selectedDiagram] = useAtom(selectedDiagramAtom);
  const diagrams = ["pie-diagram", "bar-diagram"];

  return (
    <div>
      <Container maxW={"26em"}>
        <InputField />
        {isLoading && (
          <Flex pt={["15px", "30px"]} justifyContent={"center"}>
            <CircularProgress size={"24px"} thickness="4px" isIndeterminate />
          </Flex>
        )}
        <Flex
          display={githubLanguages.size && !isLoading ? "flex" : "none"}
          flexDir={"column"}
          pt={["15px", "30px"]}
          gap={["30px", "15px"]}
          alignItems={"center"}
          pos={"relative"}
        >
          <DiagramButtons />
          <Flex
            justifyContent={"center"}
            alignItems={"center"}
            maxW={"26em"}
            w={"100%"}
          >
            {diagrams.map((id) => {
              return (
                <DiagramCanvas
                  key={id}
                  id={id}
                  isActive={id === selectedDiagram}
                />
              );
            })}
          </Flex>
          <LanguageList />
        </Flex>
      </Container>
    </div>
  );
};
