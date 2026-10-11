//! Mirror of the Ice Barrier
//! 氷結界の鏡
use super::prelude::*;

pub const ID: u32 = 10691144;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_REMOVE,nil,1,1-tp,LOCATION_HAND|LOCATION_ONFIELD|LOCATION_GRAVE)
    duel::set_possible_operation_info(
        d,
        0,
        CATEGORY_REMOVE,
        None,
        1,
        1 - a.tp,
        (LOCATION_HAND | LOCATION_ONFIELD | LOCATION_GRAVE) as i32,
    );
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetFlagEffect(tp,id)~=0 then return end
    if duel::get_flag_effect(d, a.tp, ID) != 0 {
        return;
    }
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_REMOVE)
    effect::set_code(d, e1, EVENT_REMOVE);
    // e1:SetCondition(s.rmcon)
    effect::set_condition(d, e1, Some(condition(rmcon)));
    // e1:SetOperation(s.rmop)
    effect::set_operation(d, e1, Some(operation(rmop)));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
    duel::register_flag_effect(d, a.tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
}
// function s.rmcon(e,tp,eg,ep,ev,re,r,rp)
fn rmcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local flag=0
    let mut flag: i64 = 0;
    // for tc in eg:Iter() do
    let eg = a.eg.clone().nil();
    for tc in group::iter(d, &eg) {
        // local ploc=tc:GetPreviousLocation()
        let ploc = card::get_previous_location(d, tc) as i64;
        // local te=tc:GetReasonEffect()
        let te = card::get_reason_effect(d, tc);
        // if tc:IsReason(REASON_EFFECT) and not tc:IsReason(REASON_REDIRECT) and (ploc&(LOCATION_HAND|LOCATION_ONFIELD|LOCATION_GRAVE))~=0
        //     and tc:IsPreviousControler(tp) and te:GetOwnerPlayer()==1-tp and te:IsMonsterEffect() and te:IsActivated() then
        if card::is_reason(d, tc, REASON_EFFECT)
            && !card::is_reason(d, tc, REASON_REDIRECT)
            && (ploc & (LOCATION_HAND | LOCATION_ONFIELD | LOCATION_GRAVE) as i64) != 0
            && card::is_previous_controler(d, tc, a.tp)
            && effect::get_owner_player(d, te.nil()) == 1 - a.tp
            && effect::is_monster_effect(d, te.nil())
            && effect::is_activated(d, te.nil())
        {
            // flag=(flag|ploc)
            flag |= ploc;
        }
    }
    // e:SetLabel(flag)
    effect::set_label(d, a.e, &[flag]);
    // return flag~=0
    flag != 0
}
// function s.rmfilter(c)
fn rmfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAbleToRemove() and aux.SpElimFilter(c)
    card::is_able_to_remove(d, c, None, None, None) && aux::sp_elim_filter(d, c, false, false)
}
// function s.rmop(e,tp,eg,ep,ev,re,r,rp)
fn rmop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Group.CreateGroup()
    let mut g = CardSet::new();
    // local flag=e:GetLabel()
    let flag = effect::get_label(d, a.e);
    // if (flag&LOCATION_HAND)~=0 then
    if (flag & LOCATION_HAND as i64) != 0 {
        // local rg=Duel.GetMatchingGroup(Card.IsAbleToRemove,tp,0,LOCATION_HAND,nil)
        let rg = duel::get_matching_group(
            d,
            Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)),
            tp,
            0,
            LOCATION_HAND,
            NO_EX,
        );
        // if #rg>0 then
        if !rg.is_empty() {
            // local ct=1
            let mut ct = 1;
            // if #rg>1 then ct=Duel.SelectOption(tp,aux.Stringid(id,3),aux.Stringid(id,4))+1 end
            if rg.len() > 1 {
                ct =
                    duel::select_option(d, tp, true, &[aux::stringid(ID, 3), aux::stringid(ID, 4)])
                        + 1;
            }
            // g:Merge(rg:RandomSelect(tp,ct))
            let sel = group::random_select(d, &rg, tp, ct as usize);
            group::add_card(&mut g, &sel);
        }
    }
    // if (flag&LOCATION_ONFIELD)~=0 then
    if (flag & LOCATION_ONFIELD as i64) != 0 {
        // local rg=Duel.GetMatchingGroup(Card.IsAbleToRemove,tp,0,LOCATION_ONFIELD,nil)
        let rg = duel::get_matching_group(
            d,
            Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)),
            tp,
            0,
            LOCATION_ONFIELD,
            NO_EX,
        );
        // if #rg>0 then
        if !rg.is_empty() {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
            // g:Merge(rg:Select(tp,1,2,nil))
            let sel = group::select(d, &rg, tp, 1, 2, false, NO_EX).nil();
            group::add_card(&mut g, &sel);
        }
    }
    // if (flag&LOCATION_GRAVE)~=0 then
    if (flag & LOCATION_GRAVE as i64) != 0 {
        // local rg=Duel.GetMatchingGroup(s.rmfilter,tp,0,LOCATION_MZONE|LOCATION_GRAVE,nil)
        let rg = duel::get_matching_group(
            d,
            Some(&rmfilter),
            tp,
            0,
            LOCATION_MZONE | LOCATION_GRAVE,
            NO_EX,
        );
        // if #rg>0 then
        if !rg.is_empty() {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
            // g:Merge(rg:Select(tp,1,2,nil))
            let sel = group::select(d, &rg, tp, 1, 2, false, NO_EX).nil();
            group::add_card(&mut g, &sel);
        }
    }
    // Duel.Remove(g,POS_FACEUP,REASON_EFFECT)
    duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None);
}
