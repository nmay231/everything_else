import asyncio


async def main():
    await first()
    await second()


async def first():
    print("first 1")
    await asyncio.sleep(0.5)
    print("first 2")


async def second():
    print("second 1")
    await asyncio.sleep(0.5)
    print("second 2")


asyncio.run(main())
