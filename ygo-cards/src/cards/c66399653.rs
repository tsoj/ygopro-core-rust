//! Union Hangar
use super::prelude::*;

pub const ID: u32 = 66399653;
pub const INFO: ScriptInfo = ScriptInfo { listed_card_types: &[TYPE_UNION], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Equip 1 appropriate LIGHT Machine Union monster from your Deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e2, aux::stringid(ID, 2));
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_CUSTOM+id)
    effect::set_code(d, e2, EVENT_CUSTOM + ID);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetCountLimit(1,0,EFFECT_COUNT_CODE_SINGLE)
    effect::set_count_limit(d, e2, 1, 0, 0, EFFECT_COUNT_CODE_SINGLE as u8);
    // e2:SetTarget(s.eqtg)
    effect::set_target(d, e2, Some(cost_target(eqtg)));
    // e2:SetOperation(s.eqop)
    effect::set_operation(d, e2, Some(operation(eqop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local g=Group.CreateGroup()
    // g:KeepAlive()
    // e2:SetLabelObject(g)
    effect::set_label_object(d, e2, Value::Group(CardSet::new()));
    // Equip register
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e3:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SUMMON_SUCCESS);
    // e3:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e3:SetRange(LOCATION_FZONE)
    effect::set_range(d, e3, LOCATION_FZONE);
    // e3:SetLabelObject(e2)
    effect::set_label_object(d, e3, Value::Effect(e2));
    // e3:SetOperation(s.regop)
    effect::set_operation(d, e3, Some(operation(regop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    // e4:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e4, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_LIGHT) and c:IsRace(RACE_MACHINE) and c:IsType(TYPE_UNION) and c:IsAbleToHand()
    card::is_attribute(d, c, ATTRIBUTE_LIGHT) && card::is_race(d, c, RACE_MACHINE) && card::is_type(d, c, TYPE_UNION) && card::is_able_to_hand(d, c, None)
}
/// `s.activate(e,tp,...)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(s.thfilter,tp,LOCATION_DECK,0,nil)
    let g = duel::get_matching_group(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, NO_EX);
    // if #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
    if !g.is_empty() && duel::select_yes_no(d, a.tp, aux::stringid(ID, 1)) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
        // local sg=g:Select(tp,1,1,nil)
        let sg = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
        // Duel.SendtoHand(sg,nil,REASON_EFFECT)
        duel::send_to_hand(d, sg.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,sg)
        duel::confirm_cards(d, 1 - a.tp, sg);
    }
}
/// `s.tgfilter(c,e,tp,chk)`
fn tgfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, chk: bool) -> bool {
    // return c:IsRace(RACE_MACHINE) and c:IsAttribute(ATTRIBUTE_LIGHT) and c:IsType(TYPE_UNION)
    //     and c:IsLocation(LOCATION_MZONE) and c:IsFaceup() and c:IsControler(tp) and c:IsCanBeEffectTarget(e)
    //     and (chk or Duel.IsExistingMatchingCard(s.eqfilter,tp,LOCATION_DECK,0,1,nil,c))
    card::is_race(d, c, RACE_MACHINE)
        && card::is_attribute(d, c, ATTRIBUTE_LIGHT)
        && card::is_type(d, c, TYPE_UNION)
        && card::is_location(d, c, LOCATION_MZONE)
        && card::is_faceup(d, c)
        && card::is_controler(d, c, tp)
        && card::is_can_be_effect_target(d, c, Some(e))
        && (chk || duel::is_existing_matching_card(d, Some(&|d, x| eqfilter(d, x, c)), tp, LOCATION_DECK, 0, 1, NO_EX))
}
/// `s.eqfilter(c,ec)`
fn eqfilter(d: &mut Duel, c: CardId, ec: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_LIGHT) and c:IsRace(RACE_MACHINE) and c:IsType(TYPE_UNION)
    //     and c:CheckUnionTarget(ec) and aux.CheckUnionEquip(c,ec) and not c:IsCode(ec:GetCode())
    // (`ec:GetCode()` may give two codes: both are passed on)
    let codes = card::get_code(d, ec);
    card::is_attribute(d, c, ATTRIBUTE_LIGHT)
        && card::is_race(d, c, RACE_MACHINE)
        && card::is_type(d, c, TYPE_UNION)
        && card::check_union_target(d, c, ec)
        && aux::check_union_equip(d, c, ec)
        && !card::is_code(d, c, &codes)
}
/// `s.regop(e,tp,eg,ep,ev,re,r,rp)`
fn regop(d: &mut Duel, a: &EffArgs) {
    // local tg=eg:Filter(s.tgfilter,nil,e,tp,false)
    let tg = group::filter(d, &a.eg(), &|d, c| tgfilter(d, c, a.e, a.tp, false), NO_EX);
    // if #tg>0 then
    if !tg.is_empty() {
        // for tc in aux.Next(tg) do
        for tc in group::iter(d, &tg) {
            // tc:RegisterFlagEffect(id,RESET_CHAIN,0,1)
            card::register_flag_effect(d, tc, ID, RESET_CHAIN, 0, 1, 0, 0);
        }
        // local g=e:GetLabelObject():GetLabelObject()
        let e2 = effect::get_label_object(d, a.e).as_effect().nil();
        let mut g = effect::get_label_object(d, e2).as_group().cloned().nil();
        // if Duel.GetCurrentChain()==0 then g:Clear() end
        if duel::get_current_chain(d, false) == 0 {
            g.clear();
        }
        // g:Merge(tg)
        group::add_card(&mut g, &tg);
        // g:Remove(function(c) return c:GetFlagEffect(id)==0 end,nil)
        group::remove(d, &mut g, &|d, c| card::get_flag_effect(d, c, ID) == 0, NO_EX);
        // e:GetLabelObject():SetLabelObject(g)
        effect::set_label_object(d, e2, Value::Group(g));
        // if Duel.GetFlagEffect(tp,id)==0 then
        if duel::get_flag_effect(d, a.tp, ID) == 0 {
            // Duel.RegisterFlagEffect(tp,id,RESET_CHAIN,0,1)
            duel::register_flag_effect(d, a.tp, ID, RESET_CHAIN, 0, 1, 0);
            // Duel.RaiseSingleEvent(e:GetHandler(),EVENT_CUSTOM+id,e,0,tp,tp,0)
            let h = effect::get_handler(d, a.e).nil();
            duel::raise_single_event(d, h, EVENT_CUSTOM + ID, Some(a.e), 0, a.tp, a.tp, 0);
        }
    }
}
/// `s.eqtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=e:GetLabelObject():Filter(s.tgfilter,nil,e,tp,false)
    let stored = effect::get_label_object(d, a.e).as_group().cloned().nil();
    let g = group::filter(d, &stored, &|d, c| tgfilter(d, c, a.e, a.tp, false), NO_EX);
    // if chkc then return g:IsContains(chkc) and s.tgfilter(chkc,e,tp,true) end
    if let Some(chkc) = a.chkc() {
        return g.contains(chkc) && tgfilter(d, chkc, a.e, a.tp, true);
    }
    // if chk==0 then
    if a.chk == Some(0) {
        // Duel.ResetFlagEffect(tp,id)
        duel::reset_flag_effect(d, a.tp, ID);
        // for tc in aux.Next(g) do tc:ResetFlagEffect(id) end
        for tc in group::iter(d, &g) {
            card::reset_flag_effect(d, tc, ID);
        }
        // return #g>0 and Duel.GetLocationCount(tp,LOCATION_SZONE)>0
        return !g.is_empty() && duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0;
    }
    // if #g==1 then Duel.SetTargetCard(g:GetFirst())
    if g.len() == 1 {
        let tc = group::get_first(d, &g).nil();
        duel::set_target_card(d, tc);
    } else {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EQUIP as u64);
        // local tc=g:Select(tp,1,1,nil)
        let tc = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
        // Duel.SetTargetCard(tc)
        duel::set_target_card(d, tc);
    }
    true
}
/// `s.eqop(e,tp,...)`
fn eqop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() and tc:IsControler(tp) then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) && card::is_controler(d, tc, a.tp) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EQUIP as u64);
        // local ec=Duel.SelectMatchingCard(tp,s.eqfilter,tp,LOCATION_DECK,0,1,1,nil,tc):GetFirst()
        let sel = duel::select_matching_card(d, a.tp, Some(&|d, x| eqfilter(d, x, tc)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        let ec = group::get_first(d, &sel);
        // if ec and Duel.Equip(tp,ec,tc) then
        if let Some(ec) = ec {
            if duel::equip(d, a.tp, ec, tc, true, false) {
                // aux.SetUnionState(ec)
                aux::set_union_state(d, ec);
                // local e1=Effect.CreateEffect(e:GetHandler())
                let h = effect::get_handler(d, a.e).nil();
                let e1 = effect::create_effect(d, h);
                // e1:SetDescription(aux.Stringid(id,3))
                effect::set_description(d, e1, aux::stringid(ID, 3));
                // e1:SetType(EFFECT_TYPE_SINGLE)
                effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
                effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
                // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
                effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
                // e1:SetReset(RESETS_STANDARD_PHASE_END)
                effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
                // ec:RegisterEffect(e1)
                card::register_effect(d, ec, e1, false);
            }
        }
    }
}
