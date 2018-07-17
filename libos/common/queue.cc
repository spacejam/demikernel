// -*- mode: c++; c-file-style: "k&r"; c-basic-offset: 4 -*-
/***********************************************************************
 *
 * common/queue.cc
 *   Basic queue implementation
 *
 * Copyright 2018 Irene Zhang  <irene.zhang@microsoft.com>
 *
 * Permission is hereby granted, free of charge, to any person
 * obtaining a copy of this software and associated documentation
 * files (the "Software"), to deal in the Software without
 * restriction, including without limitation the rights to use, copy,
 * modify, merge, publish, distribute, sublicense, and/or sell copies
 * of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be
 * included in all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
 * EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
 * MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
 * NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
 * BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
 * ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 **********************************************************************/
 
#include "common/queue.h"

namespace Zeus {
    
ssize_t
Queue::push(qtoken qt, struct sgarray &sga)
{
    size_t len = 0;
    for (int i = 0; i < sga.num_bufs; i++) {
        size += sga.bufs[i].len;
    }
   
    std::lock_guard<std::mutex> lock(qLock);
    if (!waiting.empty()) {
        qtoken pop = waiting.front();
        waiting.pop_front();
        auto it = wakeup.find(pop);
        if (it != wakeup.end()) {
            it->second.notify_all();
        }
        finished[pop] = sga;
    } else {
        buffer.push_back(sga);
    }
    return len;
}
        
ssize_t
Queue::pop(qtoken qt, struct sgarray &sga) {
    size_t len = 0;
    std::lock_guard<std::mutex> lock(qLock);
    // if we have a buffered push already
    if (!buffer.empty()) {
        len = sga.copy(buffer.front());
        buffer.pop_front();
    } else {
        waiting.push_back(qt);
    }
    return len;
}
   
ssize_t
Queue::wait(qtoken qt, struct sgarray &sga)
{
    std::lock_guard<std::mutex> lock(qLock);
    while ((auto it = finished.find(qt)) == finished.end()) {
        wakeup[qt].wait();
    }
    wakeup.erase(it);
    size_t len = sga.copy(it->second);
    finished.erase(it);
    return len;
}
    
ssize_t
Queue::poll(qtoken qt, struct sgarray &sga)
{
    size_t len = 0;
    std::lock_guard<std::mutex> lock(qLock);
    auto it = finished.find(qt);
    if (it != finished.end()) {
        len = sga.copy(it->second);
        finished.erase(it);
    }
    return len;
} // namespace Zeus



       
