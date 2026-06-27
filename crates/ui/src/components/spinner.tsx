import { forwardRef, HTMLProps } from "react";

type SpinnerSize = "small" | "medium" | number;

export type SpinnerProps = {
  size?: SpinnerSize;
} & HTMLProps<HTMLDivElement>;

const getSize = (size: SpinnerSize): number => {
  switch (size) {
    case "small":
      return 20;
    case "medium":
      return 30;
    default:
      return size;
  }
};

// eslint-disable-next-line react/display-name
export const Spinner = forwardRef<HTMLDivElement, SpinnerProps>(
  ({ size = 50, ...props }, ref) => (
    <div
      className="shrink-0 flex justify-center items-center overflow-hidden"
      {...props}
      ref={ref}
    >
      <div
        className="animate-spin border-[3px] border-t-[3px] border-t-[#396cd8] rounded-full"
        style={{ width: `${getSize(size)}px`, height: `${getSize(size)}px` }}
      />
    </div>
  ),
);
