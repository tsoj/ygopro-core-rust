//! Magical Hats
use super::prelude::*;

pub const ID: u32 = 81210420;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_POSITION + CATEGORY_SPECIAL_SUMMON + CATEGORY_SET);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, 0, Some(TIMING_BATTLE_START | TIMING_BATTLE_END));
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
/// `s.condition`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(1-tp) and Duel.IsBattlePhase()
    duel::is_turn_player(d, 1 - a.tp) && duel::is_battle_phase(d, None)
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:GetSequence()<5 and not c:IsType(TYPE_TOKEN+TYPE_LINK)
    card::get_sequence(d, c) < 5 && !card::is_type(d, c, TYPE_TOKEN + TYPE_LINK)
}
/// `s.spfilter(c,e,tp)` (the Lua filter does not use `e`)
fn spfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSpellTrap() and Duel.IsPlayerCanSpecialSummonMonster(tp,c:GetCode(),nil,TYPE_MONSTER|TYPE_NORMAL,0,0,0,0,0,POS_FACEDOWN)
    card::is_spell_trap(d, c) && {
        let code = card::get_code(d, c).first().copied().nil();
        let md = duel::MonsterData {
            type_: Some(TYPE_MONSTER | TYPE_NORMAL),
            attack: Some(0),
            defense: Some(0),
            level: Some(0),
            race: Some(0),
            attribute: Some(0),
            ..Default::default()
        };
        duel::is_player_can_special_summon_monster(d, tp, code, md, Some(POS_FACEDOWN as u8), None, 0)
    }
}
/// `s.target`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_MZONE,0,1,nil)
    //     and not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
    //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>1
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK,0,2,nil,e,tp)
    // end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&filter), tp, LOCATION_MZONE, 0, 1, NO_EX)
            && duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_none()
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 1
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, tp)), tp, LOCATION_DECK, 0, 2, NO_EX);
    }
    // --Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,2,tp,LOCATION_DECK)
    true
}
/// `s.activate`
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then return end
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        return;
    }
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<2 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 < 2 {
        return;
    }
    // local g=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_DECK,0,nil,e,tp)
    let g = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, tp)), tp, LOCATION_DECK, 0, NO_EX);
    // if #g<2 then return end
    if g.len() < 2 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_MZONE,0,1,1,nil):GetFirst()
    let sel = duel::select_matching_card(d, tp, Some(&filter), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &sel);
    // if not tc or tc:IsImmuneToEffect(e) then return end
    let Some(tc) = tc else { return };
    if card::is_immune_to_effect(d, tc, e) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local sg=g:Select(tp,2,2,nil)
    let mut sg = group::select(d, &g, tp, 2, 2, false, NO_EX).nil();
    // if tc:IsFaceup() then
    if card::is_faceup(d, tc) {
        // if tc:IsHasEffect(EFFECT_LIGHT_OF_INTERVENTION) then Duel.ChangePosition(tc,POS_FACEUP_DEFENSE)
        if !card::is_has_effect(d, tc, EFFECT_LIGHT_OF_INTERVENTION, None).is_empty() {
            duel::change_position(d, tc, POS_FACEUP_DEFENSE as u8, None, None, None, false);
        // else
        } else {
            // Duel.ChangePosition(tc,POS_FACEDOWN_DEFENSE)
            duel::change_position(d, tc, POS_FACEDOWN_DEFENSE as u8, None, None, None, false);
            // tc:ClearEffectRelation()
            card::clear_effect_relation(d, tc);
        }
    }
    // local fid=e:GetHandler():GetFieldID()
    let h = effect::get_handler(d, e).nil();
    let fid = card::get_field_id(d, h);
    // for tg in aux.Next(sg) do
    for tg in group::iter(d, &sg) {
        // local e1=Effect.CreateEffect(tg)
        let e1 = effect::create_effect(d, tg);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_CHANGE_TYPE);
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        effect::set_value(d, e1, (TYPE_NORMAL + TYPE_MONSTER) as i64);
        effect::set_reset(d, e1, RESET_EVENT | RESET_TOGRAVE | RESET_REMOVE | RESET_TEMP_REMOVE | RESET_TOHAND | RESET_TODECK | RESET_OVERLAY, 1);
        // tg:RegisterEffect(e1,true)
        card::register_effect(d, tg, e1, true);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        effect::set_code(d, e2, EFFECT_REMOVE_RACE);
        effect::set_value(d, e2, RACE_ALL as i64);
        card::register_effect(d, tg, e2, true);
        // local e3=e1:Clone()
        let e3 = effect::clone(d, e1);
        effect::set_code(d, e3, EFFECT_REMOVE_ATTRIBUTE);
        effect::set_value(d, e3, 0xff);
        card::register_effect(d, tg, e3, true);
        // local e4=e1:Clone()
        let e4 = effect::clone(d, e1);
        effect::set_code(d, e4, EFFECT_SET_BASE_ATTACK);
        effect::set_value(d, e4, 0);
        card::register_effect(d, tg, e4, true);
        // local e5=e1:Clone()
        let e5 = effect::clone(d, e1);
        effect::set_code(d, e5, EFFECT_SET_BASE_DEFENSE);
        effect::set_value(d, e5, 0);
        card::register_effect(d, tg, e5, true);
        // tg:RegisterFlagEffect(id,RESET_EVENT|...|RESET_PHASE|PHASE_BATTLE,0,1,fid)
        card::register_flag_effect(
            d,
            tg,
            ID,
            RESET_EVENT | RESET_TOGRAVE | RESET_REMOVE | RESET_TEMP_REMOVE | RESET_TOHAND | RESET_TODECK | RESET_OVERLAY | RESET_PHASE | PHASE_BATTLE,
            0,
            1,
            fid as i64,
            0,
        );
        // tg:SetStatus(STATUS_NO_LEVEL,true)
        card::set_status(d, tg, STATUS_NO_LEVEL, true);
    }
    // Duel.SpecialSummon(sg,0,tp,tp,true,false,POS_FACEDOWN_DEFENSE)
    duel::special_summon(d, sg.clone(), 0, tp, tp, true, false, POS_FACEDOWN_DEFENSE as u8, None);
    // Duel.ConfirmCards(1-tp,sg)
    duel::confirm_cards(d, 1 - tp, sg.clone());
    // sg:AddCard(tc)
    group::add_card(&mut sg, &CardSet::from_card(tc));
    // Duel.ShuffleSetCard(sg)
    duel::shuffle_set_card(d, &sg);
    // sg:RemoveCard(tc)
    group::remove_card(&mut sg, &CardSet::from_card(tc));
    // sg:KeepAlive() (groups are values in Rust)
    // local de=Effect.CreateEffect(e:GetHandler())
    let de = effect::create_effect(d, h);
    // de:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, de, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // de:SetCode(EVENT_PHASE|PHASE_BATTLE)
    effect::set_code(d, de, EVENT_PHASE | PHASE_BATTLE);
    // de:SetReset(RESET_PHASE|PHASE_BATTLE)
    effect::set_reset(d, de, RESET_PHASE | PHASE_BATTLE, 1);
    // de:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, de, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // de:SetCountLimit(1)
    effect::set_count_limit(d, de, 1, 0, 0, 0);
    // de:SetLabel(fid)
    effect::set_label(d, de, &[fid as i64]);
    // de:SetLabelObject(sg)
    effect::set_label_object(d, de, Value::Group(sg));
    // de:SetOperation(s.desop)
    effect::set_operation(d, de, Some(operation(desop)));
    // Duel.RegisterEffect(de,tp)
    duel::register_effect(d, de, tp);
}
/// `s.desfilter(c,fid)`
fn desfilter(d: &mut Duel, c: CardId, fid: i64) -> bool {
    // return c:GetFlagEffectLabel(id)==fid
    card::get_flag_effect_label(d, c, ID).first().copied() == Some(fid)
}
/// `s.desop`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local g=e:GetLabelObject()
    let g = effect::get_label_object(d, a.e).as_group().cloned().nil();
    // local fid=e:GetLabel()
    let fid = effect::get_label(d, a.e);
    // local tg=g:Filter(s.desfilter,nil,fid)
    let tg = group::filter(d, &g, &|d, c| desfilter(d, c, fid), NO_EX);
    // g:DeleteGroup() (groups are values in Rust)
    // Duel.Destroy(tg,REASON_EFFECT)
    duel::destroy(d, tg.clone(), REASON_EFFECT, None, None);
    // local tg2=tg:Filter(s.desfilter,nil,fid)
    let tg2 = group::filter(d, &tg, &|d, c| desfilter(d, c, fid), NO_EX);
    // Duel.SendtoGrave(tg2,REASON_EFFECT)
    duel::send_to_grave(d, tg2, REASON_EFFECT, None, None);
}
