//! Peaceful Planet Calarium
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 82460246;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_VISAS_STARFROST], listed_series: &[SET_MANNADIUM], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate and add 1 "Mannadium" monster or "Visas Starfrost" to the hand
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
    // Increase the ATK of LIGHT monsters you control by 100 per each Tuner
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    // e2:SetTarget(aux.TargetBoolFunction(Card.IsAttribute,ATTRIBUTE_LIGHT))
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| card::is_attribute(d, c, ATTRIBUTE_LIGHT))));
    // e2:SetValue(s.atkvalue)
    effect::set_value_fn(d, e2, lua_fn(atkvalue));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Special Summon 1 destroyed monster
    // local e3a=Effect.CreateEffect(c)
    let e3a = effect::create_effect(d, c);
    // e3a:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3a, aux::stringid(ID, 1));
    // e3a:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3a, CATEGORY_SPECIAL_SUMMON);
    // e3a:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3a, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3a:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DELAY)
    effect::set_property(d, e3a, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    // e3a:SetCode(EVENT_CUSTOM+id)
    effect::set_code(d, e3a, EVENT_CUSTOM + ID);
    // e3a:SetRange(LOCATION_FZONE)
    effect::set_range(d, e3a, LOCATION_FZONE);
    // e3a:SetCountLimit(1,id)
    effect::set_count_limit(d, e3a, 1, ID, 0, 0);
    // e3a:SetTarget(s.sptg)
    effect::set_target(d, e3a, Some(cost_target(sptg)));
    // e3a:SetOperation(s.spop)
    effect::set_operation(d, e3a, Some(operation(spop)));
    // c:RegisterEffect(e3a)
    card::register_effect(d, c, e3a, false);
    // local g=Group.CreateGroup()
    // g:KeepAlive()
    // e3a:SetLabelObject(g)
    effect::set_label_object(d, e3a, Value::Group(CardSet::new()));
    // Register the destuction of monsters
    // local e3b=Effect.CreateEffect(c)
    let e3b = effect::create_effect(d, c);
    // e3b:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3b, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e3b:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e3b, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e3b:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e3b, EVENT_DESTROYED);
    // e3b:SetRange(LOCATION_FZONE)
    effect::set_range(d, e3b, LOCATION_FZONE);
    // e3b:SetLabelObject(e3a)
    effect::set_label_object(d, e3b, Value::Effect(e3a));
    // e3b:SetOperation(s.regop)
    effect::set_operation(d, e3b, Some(operation(regop)));
    // c:RegisterEffect(e3b)
    card::register_effect(d, c, e3b, false);
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return ((c:IsMonster() and c:IsSetCard(SET_MANNADIUM)) or c:IsCode(CARD_VISAS_STARFROST)) and c:IsAbleToHand()
    ((card::is_monster(d, c) && card::is_set_card(d, c, &[SET_MANNADIUM as u16])) || card::is_code(d, c, &[CARD_VISAS_STARFROST]))
        && card::is_able_to_hand(d, c, None)
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(s.thfilter,tp,LOCATION_DECK,0,nil)
    let g = duel::get_matching_group(d, Some(&thfilter), tp, LOCATION_DECK, 0, NO_EX);
    // if #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,0)) then
    if !g.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 0)) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local sg=g:Select(tp,1,1,nil)
        let sg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        // Duel.SendtoHand(sg,nil,REASON_EFFECT)
        duel::send_to_hand(d, &sg, None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,sg)
        duel::confirm_cards(d, 1 - tp, &sg);
    }
}
/// `s.atkvalue(e)`
fn atkvalue(d: &mut Duel, a: Args) -> i64 {
    // return Duel.GetMatchingGroupCount(aux.FaceupFilter(Card.IsType,TYPE_TUNER),e:GetHandlerPlayer(),LOCATION_MZONE|LOCATION_GRAVE,0,nil)*100
    let p = effect::get_handler_player(d, a.e(0));
    duel::get_matching_group_count(d, Some(&|d, c| card::is_faceup(d, c) && card::is_type(d, c, TYPE_TUNER)), p, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX) as i64 * 100
}
/// `s.tgfilter(c,tp,e)`
fn tgfilter(d: &mut Duel, c: CardId, tp: u8, e: EffectId) -> bool {
    // return c:IsPreviousLocation(LOCATION_MZONE) and c:IsPreviousPosition(POS_FACEUP)
    //   and c:IsLocation(LOCATION_GRAVE|LOCATION_REMOVED) and c:IsFaceup()
    //   and c:GetPreviousTypeOnField()&TYPE_TUNER>0 and c:IsPreviousControler(tp)
    //   and c:IsReason(REASON_BATTLE|REASON_EFFECT) and c:IsCanBeEffectTarget(e)
    //   and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_previous_location(d, c, LOCATION_MZONE)
        && card::is_previous_position(d, c, POS_FACEUP)
        && card::is_location(d, c, LOCATION_GRAVE | LOCATION_REMOVED)
        && card::is_faceup(d, c)
        && (card::get_previous_type_on_field(d, c) & TYPE_TUNER) > 0
        && card::is_previous_controler(d, c, tp)
        && card::is_reason(d, c, REASON_BATTLE | REASON_EFFECT)
        && card::is_can_be_effect_target(d, c, Some(e))
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // local g=e:GetLabelObject():Filter(s.tgfilter,nil,tp,e)
    let lg = effect::get_label_object(d, e).as_group().cloned().nil();
    let g = group::filter(d, &lg, &|d, c| tgfilter(d, c, tp, e), NO_EX);
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and #g>0 end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 && !g.is_empty();
    }
    // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
    duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
    // local tc=nil
    // if #g==1 then
    if g.len() == 1 {
        // tc=g:GetFirst()
        let tc = group::get_first(d, &g).nil();
        // Duel.SetTargetCard(tc)
        duel::set_target_card(d, tc);
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,tc,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(tc)), 1, tp, 0);
    } else {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
        // tc=g:Select(tp,1,1,nil)
        let tc = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        // Duel.SetTargetCard(tc)
        duel::set_target_card(d, &tc);
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,tc,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(tc), 1, tp, 0);
    }
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, CardSet::from_card(tc), 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.regop(e,tp,eg,ep,ev,re,r,rp)`
fn regop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetFlagEffect(tp,id)>0 then return end
    if duel::get_flag_effect(d, tp, ID) > 0 {
        return;
    }
    // local tg=eg:Filter(s.tgfilter,nil,tp,e)
    let eg = a.eg.clone().nil();
    let e = a.e;
    let tg = group::filter(d, &eg, &|d, c| tgfilter(d, c, tp, e), NO_EX);
    // if #tg>0 then
    if !tg.is_empty() {
        // for tc in tg:Iter() do
        for tc in group::iter(d, &tg) {
            // tc:RegisterFlagEffect(id,RESET_CHAIN,0,1)
            card::register_flag_effect(d, tc, ID, RESET_CHAIN, 0, 1, 0, 0);
        }
        // local g=e:GetLabelObject():GetLabelObject()
        let e3a = effect::get_label_object(d, e).as_effect().nil();
        let mut g = effect::get_label_object(d, e3a).as_group().cloned().nil();
        // if Duel.GetCurrentChain()==0 then g:Clear() end
        if duel::get_current_chain(d, false) == 0 {
            g = CardSet::new();
        }
        // g:Merge(tg)
        group::add_card(&mut g, &tg);
        // g:Remove(function(c) return c:GetFlagEffect(id)==0 end,nil)
        group::remove(d, &mut g, &|d, c| card::get_flag_effect(d, c, ID) == 0, NO_EX);
        // e:GetLabelObject():SetLabelObject(g)
        effect::set_label_object(d, e3a, Value::Group(g));
        // Duel.RaiseSingleEvent(e:GetHandler(),EVENT_CUSTOM+id,e,0,tp,tp,0)
        let h = effect::get_handler(d, e).nil();
        duel::raise_single_event(d, h, EVENT_CUSTOM + ID, Some(e), 0, tp, tp, 0);
    }
}
