import { Flex, IconButton } from "@chakra-ui/react";
import { AiOutlineBarChart } from "react-icons/ai";
import { BsPieChart } from "react-icons/bs";
import { useAtom } from "jotai";
import { selectedDiagramAtom } from "@shared/model/diagram";
import { useMemo } from "react";

export const View = () => {
  const [, setSelectedDiagram] = useAtom(selectedDiagramAtom);

  const buttons = useMemo(
    () => [
      {
        ariaLabel: "select pie diagram",
        icon: <BsPieChart />,
        onClick: () => setSelectedDiagram("pie-diagram"),
      },
      {
        ariaLabel: "select bar diagram",
        icon: <AiOutlineBarChart />,
        onClick: () => setSelectedDiagram("bar-diagram"),
      },
    ],
    []
  );

  return (
    <Flex
      pos={["relative", "absolute"]}
      top={[0, "30px"]}
      left={{ base: "0", md: "calc(50% - 270px)" }}
      transform={{ base: "none", md: "translateX(-50%)" }}
      flexDir={["row", "column"]}
      gap={["30px", "15px"]}
    >
      {buttons.map((button, index) => {
        const { ariaLabel, icon, onClick } = button;

        return (
          <IconButton
            onClick={onClick}
            key={index}
            aria-label={ariaLabel}
            icon={icon}
          />
        );
      })}
    </Flex>
  );
};
