// Copyright (c) 2016 The vulkano developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

use smallvec::SmallVec;

use command_buffer::cb::AddCommand;
use command_buffer::cb::UnsafeCommandBufferBuilder;
use command_buffer::pool::CommandPool;
use VulkanObject;
use VulkanPointers;
use vk;

/// Wraps around a commands list and adds at the end of it a command that clears framebuffer
/// attachments.
pub struct CmdClearAttachments {
    // The attachments to clear.
    attachments: SmallVec<[vk::ClearAttachment; 8]>,
    // The rectangles to clear.
    rects: SmallVec<[vk::ClearRect; 4]>,
}

// TODO: add constructor

unsafe impl<'a, P> AddCommand<&'a CmdClearAttachments> for UnsafeCommandBufferBuilder<P>
    where P: CommandPool
{
    type Out = UnsafeCommandBufferBuilder<P>;

    #[inline]
    fn add(self, command: &'a CmdClearAttachments) -> Self::Out {
        unsafe {
            let vk = self.device().pointers();
            let cmd = self.internal_object();

            vk.CmdClearAttachments(cmd, command.attachments.len() as u32,
                                   command.attachments.as_ptr(), command.rects.len() as u32,
                                   command.rects.as_ptr());
        }

        self
    }
}
