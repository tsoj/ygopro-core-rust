//! Galaxy Photon Dragon
//! Scripted by Larry126
use super::prelude::*;

pub const ID: u32 = 85747929;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_PHOTON, SET_GALAXY],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon
    // Xyz.AddProcedure(c,nil,4,2)
    xyz::add_procedure(d, c, None, Some(4), 2, None, None, None, None, false, None);
    // Increase ATK
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetTarget(function(e,c) return c:IsAttribute(ATTRIBUTE_LIGHT) and c~=e:GetHandler() end)
    effect::set_target(d, e1, Some(effect_card_fn(atk_target)));
    // e1:SetValue(500)
    effect::set_value(d, e1, 500);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Search or send to the GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_TOGRAVE)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_TOGRAVE);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCost(Cost.DetachFromSelf(1,1,nil))
    effect::set_cost(d, e2, Some(cost::detach_from_self(cost::Count::N(1), Some(cost::Count::N(1)), None)));
    // e2:SetTarget(s.target)
    effect::set_target(d, e2, Some(cost_target(target)));
    // e2:SetOperation(s.operation)
    effect::set_operation(d, e2, Some(operation(op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Change levels to 4 or 8
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_LVCHANGE)
    effect::set_category(d, e3, CATEGORY_LVCHANGE);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_CUSTOM+id)
    effect::set_code(d, e3, EVENT_CUSTOM + ID);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetTarget(s.lvtg)
    effect::set_target(d, e3, Some(cost_target(lvtg)));
    // e3:SetOperation(s.lvop)
    effect::set_operation(d, e3, Some(operation(lvop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local g=Group.CreateGroup()
    // g:KeepAlive()
    // e3:SetLabelObject(g)
    effect::set_label_object(d, e3, Value::Group(CardSet::new()));
    // Register Special Summon of LIGHT monsters
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e4:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e4, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e4:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e4, EVENT_SPSUMMON_SUCCESS);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetLabelObject(e3)
    effect::set_label_object(d, e4, Value::Effect(e3));
    // e4:SetOperation(s.regop)
    effect::set_operation(d, e4, Some(operation(regop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
fn atk_target(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // c:IsAttribute(ATTRIBUTE_LIGHT) and c~=e:GetHandler()
    card::is_attribute(d, c, ATTRIBUTE_LIGHT) && c != effect::get_handler(d, e).nil()
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard({SET_PHOTON,SET_GALAXY}) and (c:IsAbleToHand() or c:IsAbleToGrave())
    card::is_set_card(d, c, &[SET_PHOTON as u16, SET_GALAXY as u16]) && (card::is_able_to_hand(d, c, None) || card::is_able_to_grave(d, c, None))
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&filter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_DECK,0,1,1,nil):GetFirst()
    let g = duel::select_matching_card(d, a.tp, Some(&filter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g);
    // aux.ToHandOrElse(tc,tp)
    aux::to_hand_or_else(d, tc.nil(), a.tp, None, None, None);
}
fn cfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsAttribute(ATTRIBUTE_LIGHT) and c:HasLevel() and c:IsControler(tp)
    //     and c:IsLocation(LOCATION_MZONE) and c:IsCanBeEffectTarget(e)
    card::is_faceup(d, c)
        && card::is_attribute(d, c, ATTRIBUTE_LIGHT)
        && card::has_level(d, c)
        && card::is_controler(d, c, tp)
        && card::is_location(d, c, LOCATION_MZONE)
        && card::is_can_be_effect_target(d, c, Some(e))
}
fn regop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    let tp = a.tp;
    // local tg=eg:Filter(s.cfilter,nil,e,tp)
    let eg = a.eg.clone().nil();
    let tg = group::filter(d, &eg, &|d, x| cfilter(d, x, e, tp), NO_EX);
    // if #tg>0 then
    if !tg.is_empty() {
        // for tc in tg:Iter() do
        for tc in group::iter(d, &tg) {
            // tc:RegisterFlagEffect(id,RESET_CHAIN,0,1)
            card::register_flag_effect(d, tc, ID, RESET_CHAIN, 0, 1, 0, 0);
        }
        // local g=e:GetLabelObject():GetLabelObject()
        let e3 = effect::get_label_object(d, e).as_effect().nil();
        let mut g = effect::get_label_object(d, e3).as_group().cloned().nil();
        // if Duel.GetCurrentChain()==0 then g:Clear() end
        if duel::get_current_chain(d, false) == 0 {
            g = CardSet::new();
        }
        // g:Merge(tg)
        group::add_card(&mut g, &tg);
        // g:Remove(function(c) return c:GetFlagEffect(id)==0 end,nil)
        group::remove(d, &mut g, &|d, x| card::get_flag_effect(d, x, ID) == 0, NO_EX);
        // e:GetLabelObject():SetLabelObject(g)
        effect::set_label_object(d, e3, Value::Group(g));
        // Duel.RaiseSingleEvent(e:GetHandler(),EVENT_CUSTOM+id,e,0,tp,tp,0)
        let h = effect::get_handler(d, e).nil();
        duel::raise_single_event(d, h, EVENT_CUSTOM + ID, Some(e), 0, tp, tp, 0);
    }
}
fn lvtg(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    let tp = a.tp;
    // local g=e:GetLabelObject():Filter(s.cfilter,nil,e,tp)
    let lg = effect::get_label_object(d, e).as_group().cloned().nil();
    let g = group::filter(d, &lg, &|d, x| cfilter(d, x, e, tp), NO_EX);
    // if chkc then return g:IsContains(chkc) and s.cfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return group::is_contains(&g, chkc) && cfilter(d, chkc, e, tp);
    }
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    // local tc=nil
    // if #g==1 then tc=g:GetFirst()
    let tc: CardSet = if g.len() == 1 {
        CardSet::from_card(group::get_first(d, &g).nil())
    } else {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
        // tc=g:Select(tp,1,1,nil)
        group::select(d, &g, tp, 1, 1, false, NO_EX).nil()
    };
    // Duel.SetTargetCard(tc)
    duel::set_target_card(d, tc.clone());
    // Duel.SetOperationInfo(0,CATEGORY_LVCHANGE,tc,1,tp,LOCATION_MZONE)
    duel::set_operation_info(d, 0, CATEGORY_LVCHANGE, Some(tc), 1, tp, LOCATION_MZONE as i32);
    true
}
fn lvop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsFaceup() and tc:IsRelateToEffect(e) and tc:HasLevel() then
    if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) && card::has_level(d, tc) {
        // local lv=Duel.AnnounceLevel(tp,4,8,5,6,7,tc:GetLevel())
        let level = card::get_level(d, tc);
        let lv = duel::announce_number_range(d, tp, Some(4), Some(8), &[5, 6, 7, level]).nil().0 as i64;
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_CHANGE_LEVEL)
        effect::set_code(d, e1, EFFECT_CHANGE_LEVEL);
        // e1:SetValue(lv)
        effect::set_value(d, e1, lv);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
