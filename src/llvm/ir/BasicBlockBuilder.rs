// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


pub struct BasicBlockBuilder<'a>
{
	context: &'a Context,
	functionReference: LLVMValueRef,
	basicBlockReference: LLVMBasicBlockRef,
	builder: Builder<'a>,
}

impl<'a> BasicBlockBuilder<'a>
{
	#[inline(always)]
	fn createBasicBlock(name: &str, context: &'a Context, functionReference: LLVMValueRef) -> BasicBlockBuilder<'a>
	{
		let name = CString::new(name.as_bytes()).unwrap();
		let basicBlockReference = unsafe { LLVMAppendBasicBlockInContext(context.reference, functionReference, name.as_ptr()) };
		
		let builder = context.builder();
		
		let this = Self
		{
			context,
			functionReference,
			basicBlockReference,
			builder,
		};
		
		this.builder.positionAtEndOfBasicBlock(this.basicBlockReference);
		
		this
	}
	
	#[inline(always)]
	pub fn newBasicBlock(&self, to: &str) -> BasicBlockBuilder<'a>
	{
		Self::createBasicBlock(to, self.context, self.functionReference)
	}
	
	#[inline(always)]
	pub fn parameterAt(&self, index: u32) -> LLVMValueRef
	{
		unsafe { LLVMGetParam(self.functionReference, index) }
	}
	
	#[inline(always)]
	pub fn parameterAtAsPointer(&self, index: u32) -> Pointer
	{
		Pointer(self.parameterAt(index))
	}
	
	pub fn returnVoid(self)
	{
		self.builder.returnVoid();
	}
	
	pub fn returnTrue(self)
	{
		self.builder.returnValue(&IntegerConstant::True);
	}
	
	pub fn returnFalse(self)
	{
		self.builder.returnValue(&IntegerConstant::False);
	}
	
	pub fn unconditionalBranch(self, to: &BasicBlockBuilder<'a>)
	{
		self.builder.unconditionalBranch(to.basicBlockReference);
	}
	
	pub fn unconditionalBranchWithCreation(self, to: &str) -> BasicBlockBuilder<'a>
	{
		let to = self.newBasicBlock(to);
		self.builder.unconditionalBranch(to.basicBlockReference);
		to
	}
	
	pub fn conditionalBranch(self, ifCondition: LLVMValueRef, thenBlock: &BasicBlockBuilder<'a>, elseBlock: &BasicBlockBuilder<'a>)
	{
		self.builder.conditionalBranch(ifCondition, thenBlock.basicBlockReference, elseBlock.basicBlockReference);
	}
	
	/// integerValueOrConstant's integer type must match IntegerConstant but the API can't easily enforce this
	pub fn switchBranch(self, integerValueOrConstant: LLVMValueRef, defaultBlock: &BasicBlockBuilder<'a>, caseBlocks: BTreeMap<IntegerConstant, BasicBlockBuilder<'a>>)
	{
		let switchInstruction = self.builder.switchBranch(integerValueOrConstant, defaultBlock.basicBlockReference, caseBlocks.len());
		for (constant, caseBlock) in caseBlocks.iter()
		{
			switchInstruction.addCase(constant, caseBlock.basicBlockReference)
		}
	}
	
	
	
	pub fn push(&self)
	{
		let ClientIdentifierType = LlvmType::Int64;
		let SubscriptionForThatClientIdentifierType = LlvmType::Int64;
		
		let SubscriberType = LlvmType::namedStruct
		(
			"Subscriber",
			false,
			vec!
			[
				ClientIdentifierType,                     // clientIdentifier
				SubscriptionForThatClientIdentifierType,  // subscriberIdentifier
			]
		);
		
		let CountType = LlvmType::Int64;
		let SubscriberPointerType = LlvmType::pointer(SubscriberType);
		
		let SubscribersType = LlvmType::namedStruct
		(
			"Subscribers",
			false,
			vec!
			[
				CountType,              // count
				SubscriberPointerType,  // subscribers (malloc'd in advance)
			]
		);
		
		let looper2_subscribers1 = StructConstant::named("Subscriber", false, vec!
		[
			AnyConstant::Integer(IntegerConstant::constantInteger64BitUnsigned(2000)),
			AnyConstant::Integer(IntegerConstant::constantInteger64BitUnsigned(3000)),
		]);
		
		let arrayPointer = self.builder.getElementPointer_PointerToStructToPointerToField(self.parameterAtAsPointer(0), 0, 1);
		let tbaaLoad = TbaaNode::Path
		{
			baseType: Box::new(TbaaNode::Struct
			{
				name: "Subscribers".to_owned(),
				fields: vec!
				[
					TbaaNodeStructField
					{
						kind: TbaaNode::long(), // usize
						offset: 0,
					},
					TbaaNodeStructField
					{
						kind: TbaaNode::any_pointer(),
						offset: 8,
					},
				],
			}),
			
			accessType: Box::new(TbaaNode::any_pointer()),
			
			offsetIntoBaseType: 0,
			
			isConstant: false,
		};
		let instruction = self.builder.load(arrayPointer, Some(PowerOfTwoThirtyTwoBit::_8), Some(tbaaLoad));
		
		let arrayIndexPointer = self.builder.getElementPointer_ArrayIndex(Pointer(instruction), 1000);
		self.builder.bitcastPointerToUnsignedCharPointer(arrayIndexPointer);
		// tail call
		self.builder.returnVoid();
		
/*

Note, when pushing subscribers, we can push one giant block, not just one at a time, ie if we have 10 subscribers, we should be able to memcpy(10 x size of subscriber)


; ModuleID = 'example.c'
source_filename = "example.c"
target datalayout = "e-m:o-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-apple-macosx10.11.0"

%struct.Subscriber = type { i64, i64 }
%struct.Subscribers = type { i64, %struct.Subscriber* }

@looper2.subscriber1 = internal unnamed_addr constant %struct.Subscriber { i64 2000, i64 3000 }, align 8

; Function Attrs: nounwind ssp uwtable
define void @looper2(%struct.Subscribers* noalias nocapture nonnull readonly) local_unnamed_addr #0 {
  %2 = getelementptr inbounds %struct.Subscribers, %struct.Subscribers* %0, i64 0, i32 1
  %3 = load %struct.Subscriber*, %struct.Subscriber** %2, align 8, !tbaa !2
  %4 = getelementptr inbounds %struct.Subscriber, %struct.Subscriber* %3, i64 1000
  %5 = bitcast %struct.Subscriber* %4 to i8*
  tail call void @llvm.memcpy.p0i8.p0i8.i64(i8* %5, i8* bitcast (%struct.Subscriber* @looper2.subscriber1 to i8*), i64 16, i32 8, i1 false), !tbaa.struct !8
  ret void
}

; Function Attrs: argmemonly nounwind
declare void @llvm.memcpy.p0i8.p0i8.i64(i8* nocapture writeonly, i8* nocapture readonly, i64, i32, i1) #1

attributes #0 = { nounwind ssp uwtable "correctly-rounded-divide-sqrt-fp-math"="false" "disable-tail-calls"="false" "less-precise-fpmad"="false" "no-frame-pointer-elim"="true" "no-frame-pointer-elim-non-leaf" "no-infs-fp-math"="false" "no-jump-tables"="false" "no-nans-fp-math"="false" "no-signed-zeros-fp-math"="false" "no-trapping-math"="false" "stack-protector-buffer-size"="8" "target-cpu"="core2" "target-features"="+cx16,+fxsr,+mmx,+sse,+sse2,+sse3,+ssse3,+x87" "unsafe-fp-math"="false" "use-soft-float"="false" }
attributes #1 = { argmemonly nounwind }

!llvm.module.flags = !{!0}
!llvm.ident = !{!1}

!0 = !{i32 1, !"PIC Level", i32 2}
!1 = !{!"clang version 4.0.0 (tags/RELEASE_400/final)"}
!2 = !{!3, !7, i64 8}
!3 = !{!"Subscribers", !4, i64 0, !7, i64 8}
!4 = !{!"long", !5, i64 0}
!5 = !{!"omnipotent char", !6, i64 0}
!6 = !{!"Simple C/C++ TBAA"}
!7 = !{!"any pointer", !5, i64 0}
!8 = !{i64 0, i64 8, !9, i64 8, i64 8, !9}
!9 = !{!10, !10, i64 0}
!10 = !{!"long long", !5, i64 0}



void looper2(struct Subscribers * restrict subscribers) __attribute__((nothrow, nonnull (1)));
void looper2(struct Subscribers * restrict subscribers)
{
	const struct Subscriber subscriber1 = { .clientIdentifier = 2000, .subscribtionIdentifier = 3000 };
	struct Subscriber * list = subscribers->list;
	*(list + 1000) = subscriber1;
}




*/
	}
}
