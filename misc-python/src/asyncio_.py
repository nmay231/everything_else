import asyncio


async def main():
    task1 = asyncio.ensure_future(first())
    task2 = asyncio.ensure_future(second())
    await task1
    await task2


async def first():
    print("first 1")
    await asyncio.sleep(1)
    print("first 2")


async def second():
    print("second 1")
    await asyncio.sleep(0.5)
    print("second 2")


asyncio.run(main())
