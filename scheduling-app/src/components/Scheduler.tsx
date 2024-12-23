import { Button, Text } from "@mantine/core";
import { useModals } from "@mantine/modals";

export const Scheduler = () => {
    const modals = useModals();
    return (
        <Button onClick={() => modals.openContextModal("scheduler", { innerProps: {} })}>
            Open a modal
        </Button>
    );
};
