export default {
    install(app) {
        // 获取当前年份
        const currentYear = new Date().getFullYear();
        app.config.globalProperties.$slogan = {
            main: 'TrialQ',
            version: 'Beta 0.0.1',
            sub: '临研数据分析『超跑引擎』全速驱动突破，效率一路狂飙！',
            year: currentYear, // 自动获取的当前年份
            email: 'trialqtool@gmail.com', // 固定的电子邮件地址
        };
        async function invokeListenerCommand(rustInvoke, command) {
            try {

                await rustInvoke(command);
            } catch (error) {
                console.error('调用错误:', error);
            }
        }

        function setupListener(rustListen, eventName, callback) {
            return rustListen(eventName, (event) => {
                callback(event.payload);
            });
        }

        app.config.globalProperties.$listenerEvent = {};
        app.config.globalProperties.$listenerEvent.invokeCommand = invokeListenerCommand;
        app.config.globalProperties.$listenerEvent.setupListener = setupListener;
    },
};
