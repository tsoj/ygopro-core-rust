//! Solfachord Harmonia
use super::prelude::*;

pub const ID: u32 = 29650040;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SOLFACHORD], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // effect
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetTarget(s.target)
    effect::set_target(d, e2, Some(cost_target(target)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local b1=s.thtg(e,tp,eg,ep,ev,re,r,rp,0)
    let b1 = thtg(d, &with_chk(a, 0));
    // local b2=s.pentg(e,tp,eg,ep,ev,re,r,rp,0)
    let b2 = pentg(d, &with_chk(a, 0));
    // local b3=s.destg(e,tp,eg,ep,ev,re,r,rp,0)
    let b3 = destg(d, &with_chk(a, 0));
    // if chk==0 then return b1 or b2 or b3 end
    if a.chk == Some(0) {
        return b1 || b2 || b3;
    }
    // local ops={}
    // local opval={}
    // local off=1
    let mut ops: Vec<u64> = Vec::new();
    let mut opval: Vec<i64> = Vec::new();
    // if b1 then
    if b1 {
        // ops[off]=aux.Stringid(id,0)
        ops.push(aux::stringid(ID, 0));
        // opval[off-1]=1
        opval.push(1);
        // off=off+1
    }
    // if b2 then
    if b2 {
        // ops[off]=aux.Stringid(id,1)
        ops.push(aux::stringid(ID, 1));
        // opval[off-1]=2
        opval.push(2);
        // off=off+1
    }
    // if b3 then
    if b3 {
        // ops[off]=aux.Stringid(id,2)
        ops.push(aux::stringid(ID, 2));
        // opval[off-1]=3
        opval.push(3);
        // off=off+1
    }
    // local op=Duel.SelectOption(tp,table.unpack(ops))
    let op = duel::select_option(d, a.tp, true, &ops);
    // local sel=opval[op]
    let sel = opval.get(op as usize).copied();
    match sel {
        // if sel==1 then
        Some(1) => {
            // e:SetCategory(CATEGORY_TOHAND)
            effect::set_category(d, a.e, CATEGORY_TOHAND);
            // e:SetOperation(s.thop)
            effect::set_operation(d, a.e, Some(operation(thop)));
            // s.thtg(e,tp,eg,ep,ev,re,r,rp,1)
            thtg(d, &with_chk(a, 1));
        }
        // elseif sel==2 then
        Some(2) => {
            // e:SetOperation(s.penop)
            effect::set_operation(d, a.e, Some(operation(penop)));
            // s.pentg(e,tp,eg,ep,ev,re,r,rp,1)
            pentg(d, &with_chk(a, 1));
        }
        // elseif sel==3 then
        Some(3) => {
            // e:SetCategory(CATEGORY_DESTROY)
            effect::set_category(d, a.e, CATEGORY_DESTROY);
            // e:SetOperation(s.desop)
            effect::set_operation(d, a.e, Some(operation(desop)));
            // s.destg(e,tp,eg,ep,ev,re,r,rp,1)
            destg(d, &with_chk(a, 1));
        }
        // else
        _ => {
            // e:SetCategory(0)
            effect::set_category(d, a.e, 0);
            // e:SetOperation(nil)
            effect::set_operation(d, a.e, None);
        }
    }
    true
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_SOLFACHORD) and c:IsType(TYPE_PENDULUM) and c:IsAbleToHand()
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_SOLFACHORD as u16]) && card::is_type(d, c, TYPE_PENDULUM) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetFlagEffect(tp,id)==0 and Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_EXTRA,0,1,nil) end
    if a.chk == Some(0) {
        return duel::get_flag_effect(d, a.tp, ID) == 0 && duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
    duel::register_flag_effect(d, a.tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_EXTRA,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
/// `s.penfilter(c)`
fn penfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_SOLFACHORD) and c:GetOriginalLevel()>0
    card::is_set_card(d, c, &[SET_SOLFACHORD as u16]) && card::get_original_level(d, c) > 0
}
/// `s.pentg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn pentg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetFlagEffect(tp,id+1)==0 and Duel.IsExistingMatchingCard(s.penfilter,tp,LOCATION_PZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::get_flag_effect(d, a.tp, ID + 1) == 0 && duel::is_existing_matching_card(d, Some(&penfilter), a.tp, LOCATION_PZONE, 0, 1, NO_EX);
    }
    // Duel.RegisterFlagEffect(tp,id+1,RESET_PHASE|PHASE_END,0,1)
    duel::register_flag_effect(d, a.tp, ID + 1, RESET_PHASE | PHASE_END, 0, 1, 0);
    true
}
/// `s.penop(e,tp,eg,ep,ev,re,r,rp)`
fn penop(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.penfilter,tp,LOCATION_PZONE,0,1,1,nil):GetFirst()
    let g = duel::select_matching_card(d, a.tp, Some(&penfilter), a.tp, LOCATION_PZONE, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g);
    // if tc then
    if let Some(tc) = tc {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let eh = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, eh);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_LSCALE)
        effect::set_code(d, e1, EFFECT_UPDATE_LSCALE);
        // e1:SetValue(tc:GetOriginalLevel())
        let lv = card::get_original_level(d, tc);
        effect::set_value(d, e1, lv as i64);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_UPDATE_RSCALE)
        effect::set_code(d, e2, EFFECT_UPDATE_RSCALE);
        // tc:RegisterEffect(e2)
        card::register_effect(d, tc, e2, false);
    }
}
/// `s.descfilter(c,f)` with `f = Card.IsOddScale`
fn descfilter_odd(d: &mut Duel, c: CardId) -> bool {
    desc_base(d, c) && card::is_odd_scale(d, c)
}
/// `s.descfilter(c,f)` with `f = Card.IsEvenScale`
fn descfilter_even(d: &mut Duel, c: CardId) -> bool {
    desc_base(d, c) && card::is_even_scale(d, c)
}
/// The part of `s.descfilter(c,f)` before `f(c)`.
fn desc_base(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_SOLFACHORD) and c:IsOriginalType(TYPE_PENDULUM) and c:IsOriginalType(TYPE_MONSTER)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_SOLFACHORD as u16]) && card::is_original_type(d, c, TYPE_PENDULUM) && card::is_original_type(d, c, TYPE_MONSTER)
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetMatchingGroup(nil,tp,0,LOCATION_ONFIELD,nil)
    let g = duel::get_matching_group(d, None, a.tp, 0, LOCATION_ONFIELD, NO_EX);
    // if chk==0 then
    if a.chk == Some(0) {
        // local pg1=Duel.GetMatchingGroup(s.descfilter,tp,LOCATION_ONFIELD,0,nil,Card.IsOddScale)
        let pg1 = duel::get_matching_group(d, Some(&descfilter_odd), a.tp, LOCATION_ONFIELD, 0, NO_EX);
        // local pg2=Duel.GetMatchingGroup(s.descfilter,tp,LOCATION_ONFIELD,0,nil,Card.IsEvenScale)
        let pg2 = duel::get_matching_group(d, Some(&descfilter_even), a.tp, LOCATION_ONFIELD, 0, NO_EX);
        // return Duel.GetFlagEffect(tp,id+2)==0 and #g>0 and
        //     (pg1:GetClassCount(Card.GetLeftScale)>=3 or pg2:GetClassCount(Card.GetLeftScale)>=3
        //     or pg1:GetClassCount(Card.GetRightScale)>=3 or pg2:GetClassCount(Card.GetRightScale)>=3)
        return duel::get_flag_effect(d, a.tp, ID + 2) == 0
            && !g.is_empty()
            && (group::get_class_count(d, &pg1, &|d, c| card::get_left_scale(d, c) as i64) >= 3
                || group::get_class_count(d, &pg2, &|d, c| card::get_left_scale(d, c) as i64) >= 3
                || group::get_class_count(d, &pg1, &|d, c| card::get_right_scale(d, c) as i64) >= 3
                || group::get_class_count(d, &pg2, &|d, c| card::get_right_scale(d, c) as i64) >= 3);
    }
    // Duel.RegisterFlagEffect(tp,id+2,RESET_PHASE|PHASE_END,0,1)
    duel::register_flag_effect(d, a.tp, ID + 2, RESET_PHASE | PHASE_END, 0, 1, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectMatchingCard(tp,nil,tp,0,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, None, a.tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.HintSelection(g)
        duel::hint_selection(d, g.clone(), true);
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
/// `s.thtg(...)` etc. called with an explicit `chk` value.
fn with_chk(a: &EffArgs, chk: i64) -> EffArgs {
    EffArgs { chk: Some(chk), extra: Vec::new(), ..a.clone() }
}
