//! Lost World
use super::prelude::*;

pub const ID: u32 = 17228908;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[17228909],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Monsters lose 500 ATK/DEF
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e2, LOCATION_MZONE, LOCATION_MZONE);
    // e2:SetTarget(aux.NOT(aux.TargetBoolFunction(Card.IsRace,RACE_DINOSAUR)))
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, mc| !card::is_race(d, mc, RACE_DINOSAUR))));
    // e2:SetValue(-500)
    effect::set_value(d, e2, -500);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e3, EFFECT_UPDATE_DEFENSE);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Special Summon 1 "Jurraegg Token"
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e4, aux::stringid(ID, 0));
    // e4:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN)
    effect::set_category(d, e4, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e4:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e4, EFFECT_FLAG_DELAY, 0);
    // e4:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e4, EVENT_SUMMON_SUCCESS);
    // e4:SetRange(LOCATION_FZONE)
    effect::set_range(d, e4, LOCATION_FZONE);
    // e4:SetCountLimit(1,0,EFFECT_COUNT_CODE_SINGLE)
    effect::set_count_limit(d, e4, 1, 0, 0, EFFECT_COUNT_CODE_SINGLE as u8);
    // e4:SetCondition(s.tkcon)
    effect::set_condition(d, e4, Some(condition(tkcon)));
    // e4:SetTarget(s.tktg)
    effect::set_target(d, e4, Some(cost_target(tktg)));
    // e4:SetOperation(s.tkop)
    effect::set_operation(d, e4, Some(operation(tkop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // local e5=e4:Clone()
    let e5 = effect::clone(d, e4);
    // e5:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e5, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // Opponent cannt target monster on the field, except Tokens
    // local e6=Effect.CreateEffect(c)
    let e6 = effect::create_effect(d, c);
    // e6:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e6, EFFECT_TYPE_FIELD);
    // e6:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE+EFFECT_FLAG_SET_AVAILABLE)
    effect::set_property(d, e6, EFFECT_FLAG_IGNORE_IMMUNE + EFFECT_FLAG_SET_AVAILABLE, 0);
    // e6:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e6, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e6:SetRange(LOCATION_FZONE)
    effect::set_range(d, e6, LOCATION_FZONE);
    // e6:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e6, LOCATION_MZONE, LOCATION_MZONE);
    // e6:SetCondition(s.tgcon)
    effect::set_condition(d, e6, Some(condition(tgcon)));
    // e6:SetTarget(aux.NOT(aux.TargetBoolFunction(Card.IsType,TYPE_TOKEN)))
    effect::set_target(d, e6, Some(effect_card_fn(|d, _e, mc| !card::is_type(d, mc, TYPE_TOKEN))));
    // e6:SetValue(aux.tgoval)
    effect::set_value_fn(d, e6, aux::tgoval());
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
    // destroy replace
    // local e7=Effect.CreateEffect(c)
    let e7 = effect::create_effect(d, c);
    // e7:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e7, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e7:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e7, EFFECT_DESTROY_REPLACE);
    // e7:SetRange(LOCATION_FZONE)
    effect::set_range(d, e7, LOCATION_FZONE);
    // e7:SetCountLimit(1)
    effect::set_count_limit(d, e7, 1, 0, 0, 0);
    // e7:SetTarget(s.reptg)
    effect::set_target(d, e7, Some(cost_target(reptg)));
    // e7:SetValue(s.repval)
    effect::set_value_fn(d, e7, lua_fn(repval));
    // e7:SetOperation(s.repop)
    effect::set_operation(d, e7, Some(operation(repop)));
    // c:RegisterEffect(e7)
    card::register_effect(d, c, e7, false);
    // local g=Group.CreateGroup()
    // g:KeepAlive()
    // e7:SetLabelObject(g)
    effect::set_label_object(d, e7, Value::Group(CardSet::new()));
}
/// `s.tkcon(e,tp,eg,ep,ev,re,r,rp)`
fn tkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(aux.FaceupFilter(Card.IsRace,RACE_DINOSAUR),1,nil,tp)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &aux::faceup_filter(|d, c| card::is_race(d, c, RACE_DINOSAUR)), 1, NO_EX)
}
/// `s.tktg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tktg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetLocationCount(1-tp,LOCATION_MZONE)>0
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,0,TYPES_TOKEN,0,0,1,RACE_DINOSAUR,ATTRIBUTE_EARTH,POS_FACEUP_DEFENSE,1-tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, None, None, None).0 > 0 && can_summon_token(d, tp);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, 0);
    true
}
/// `Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,0,TYPES_TOKEN,0,0,1,RACE_DINOSAUR,ATTRIBUTE_EARTH,POS_FACEUP_DEFENSE,1-tp)`
fn can_summon_token(d: &mut Duel, tp: u8) -> bool {
    duel::is_player_can_special_summon_monster(
        d,
        tp,
        ID + 1,
        duel::MonsterData {
            setcodes: Some(vec![0]),
            type_: Some(TYPES_TOKEN),
            attack: Some(0),
            defense: Some(0),
            level: Some(1),
            race: Some(RACE_DINOSAUR),
            attribute: Some(ATTRIBUTE_EARTH),
        },
        Some(POS_FACEUP_DEFENSE as u8),
        Some(1 - tp),
        0,
    )
}
/// `s.tkop(e,tp,eg,ep,ev,re,r,rp)`
fn tkop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if e:GetHandler():IsRelateToEffect(e) and Duel.GetLocationCount(1-tp,LOCATION_MZONE)>0
    //     and Duel.IsPlayerCanSpecialSummonMonster(...) then
    let h = effect::get_handler(d, e).nil();
    if card::is_relate_to_effect(d, h, e)
        && duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && can_summon_token(d, tp)
    {
        // local token=Duel.CreateToken(tp,id+1)
        let token = duel::create_token(d, tp, ID + 1).nil();
        // Duel.SpecialSummon(token,0,tp,1-tp,false,false,POS_FACEUP_DEFENSE)
        duel::special_summon(d, token, 0, tp, 1 - tp, false, false, POS_FACEUP_DEFENSE as u8, None);
    }
}
/// `s.tgcon(e)`
fn tgcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.tknfilter,e:GetHandlerPlayer(),0,LOCATION_ONFIELD,1,nil)
    let hp = effect::get_handler_player(d, a.e);
    duel::is_existing_matching_card(d, Some(&tknfilter), hp, 0, LOCATION_ONFIELD, 1, NO_EX)
}
/// `s.tknfilter(c)`
fn tknfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsType(TYPE_TOKEN) or c:IsOriginalType(TYPE_TOKEN)
    card::is_type(d, c, TYPE_TOKEN) || card::is_original_type(d, c, TYPE_TOKEN)
}
/// `s.repfilter(c,tp)`
fn repfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_NORMAL) and c:IsLocation(LOCATION_MZONE) and not c:IsReason(REASON_REPLACE)
    //     and c:IsReason(REASON_BATTLE|REASON_EFFECT) and not c:HasFlagEffect(id)
    let _ = tp;
    card::is_faceup(d, c)
        && card::is_type(d, c, TYPE_NORMAL)
        && card::is_location(d, c, LOCATION_MZONE)
        && !card::is_reason(d, c, REASON_REPLACE)
        && card::is_reason(d, c, REASON_BATTLE | REASON_EFFECT)
        && !card::has_flag_effect(d, c, ID, None)
}
/// `s.desfilter(c,e)`
fn desfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsRace(RACE_DINOSAUR) and c:IsDestructable(e) and not c:IsStatus(STATUS_DESTROY_CONFIRMED+STATUS_BATTLE_DESTROYED)
    card::is_race(d, c, RACE_DINOSAUR)
        && card::is_destructable(d, c, Some(e))
        && !card::is_status(d, c, STATUS_DESTROY_CONFIRMED + STATUS_BATTLE_DESTROYED)
}
/// `s.reptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local ct=eg:FilterCount(s.repfilter,nil,tp)
    let eg = a.eg.clone().nil();
    let ct = group::filter_count(d, &eg, &|d, c| repfilter(d, c, tp), NO_EX) as u32;
    // if chk==0 then return ct>0 and Duel.IsExistingMatchingCard(s.desfilter,tp,LOCATION_HAND|LOCATION_DECK,0,ct,nil,e) end
    if a.chk == Some(0) {
        return ct > 0 && duel::is_existing_matching_card(d, Some(&|d, c| desfilter(d, c, e)), tp, LOCATION_HAND | LOCATION_DECK, 0, ct, NO_EX);
    }
    // if Duel.SelectEffectYesNo(tp,e:GetHandler(),96) then
    let h = effect::get_handler(d, e).nil();
    if duel::select_effect_yes_no(d, tp, h, Some(96)) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESREPLACE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESREPLACE as u64);
        // local tg=Duel.SelectMatchingCard(tp,s.desfilter,tp,LOCATION_HAND|LOCATION_DECK,0,ct,ct,nil,e)
        let tg = duel::select_matching_card(d, tp, Some(&|d, c| desfilter(d, c, e)), tp, LOCATION_HAND | LOCATION_DECK, 0, ct as u16, ct as u16, false, NO_EX).nil();
        // local g=e:GetLabelObject()
        // g:Clear()
        let mut g = CardSet::new();
        // for tc in tg:Iter() do
        for tc in group::iter(d, &tg) {
            // tc:RegisterFlagEffect(id,RESET_EVENT|(RESETS_STANDARD&~RESET_TURN_SET)|RESET_CHAIN,0,1)
            card::register_flag_effect(d, tc, ID, RESET_EVENT | (RESETS_STANDARD & !RESET_TURN_SET) | RESET_CHAIN, 0, 1, 0, 0);
            // tc:SetStatus(STATUS_DESTROY_CONFIRMED,true)
            card::set_status(d, tc, STATUS_DESTROY_CONFIRMED, true);
            // g:AddCard(tc)
            group::add_card(&mut g, &CardSet::from_card(tc));
        }
        // The label group is changed in place: store it again.
        effect::set_label_object(d, e, Value::Group(g));
        // return true
        true
    } else {
        // else return false end
        false
    }
}
/// `s.repval(e,c)`
fn repval(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let c = a.c(1);
    // return s.repfilter(c,e:GetHandlerPlayer())
    let hp = effect::get_handler_player(d, e);
    repfilter(d, c, hp)
}
/// `s.repop(e,tp,eg,ep,ev,re,r,rp)`
fn repop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // Duel.Hint(HINT_CARD,1-tp,id)
    duel::hint(d, HINT_CARD as u8, 1 - tp, ID as u64);
    // local tg=e:GetLabelObject()
    let tg = effect::get_label_object(d, e).as_group().cloned().nil();
    // for tc in tg:Iter() do
    for tc in group::iter(d, &tg) {
        // tc:SetStatus(STATUS_DESTROY_CONFIRMED,false)
        card::set_status(d, tc, STATUS_DESTROY_CONFIRMED, false);
    }
    // Duel.Destroy(tg,REASON_EFFECT|REASON_REPLACE)
    duel::destroy(d, tg, REASON_EFFECT | REASON_REPLACE, None, None);
}
