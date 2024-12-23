import "./App.css";
import { MantineProvider } from "@mantine/core";
import { Scheduler } from "./components/Scheduler";
import { ModalsProvider } from "@mantine/modals";
import { SchedulerModal } from "./components/ScheduleModal";

function App() {
    return (
        <MantineProvider withGlobalStyles withNormalizeCSS>
            <ModalsProvider modals={{ scheduler: SchedulerModal }}>
                <Scheduler />
            </ModalsProvider>
        </MantineProvider>
    );
}

export default App;
