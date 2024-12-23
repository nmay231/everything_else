import { Button, Tuple } from "@mantine/core";
import { DatePicker, TimeRangeInput, TimeInput } from "@mantine/dates";
import { useForm } from "@mantine/form";
import { ContextModalProps } from "@mantine/modals";
import { useState } from "react";

export const SchedulerModal: React.FC<ContextModalProps> = () => {
    const form = useForm({
        initialValues: {
            date: null as null | Date,
            time: null as null | Date,
        },
        validate(values) {
            return {
                date: values.date ? null : "Date is required",
                time: values.time ? null : "Time is required",
            };
            // const result = {} as any;
            // if (!values.date) result.date = "Date is required";
            // if (!values.time) result.time = "Time is required";
            // return result;
        },
        validateInputOnBlur: true,
    });
    // const [date, setDate] = useState<null | Date>(null);
    // const [time, setTime] = useState<null | Date>(null);
    // const [time, setTime] = useState<Tuple<null | Date, 2>>([null, null]);

    return (
        <>
            <form
                onSubmit={form.onSubmit((values) => {
                    console.log(values);
                })}
            >
                <DatePicker
                    required
                    label="What day are you available?"
                    {...form.getInputProps("date")}
                />
                <TimeInput required label="And what time?" {...form.getInputProps("time")} />
                <Button type="submit" mt="md">
                    Submit
                </Button>
            </form>
        </>
    );
};
