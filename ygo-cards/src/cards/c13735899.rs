//! Mysterion the Dragon Crown
use super::prelude::*;

pub const ID: u32 = 13735899;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion Materials: 1 Spellcaster monster + 1 Dragon monster
    // Fusion.AddProcMix(c,true,true,aux.FilterBoolFunctionEx(Card.IsRace,RACE_SPELLCASTER),aux.FilterBoolFunctionEx(Card.IsRace,RACE_DRAGON))
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::F(fusion::fus_mat(|d, x, fc, st, tp| card::is_race_ctx(d, x, RACE_SPELLCASTER, Some(fc), st, Some(tp)))),
            fusion::Mat::F(fusion::fus_mat(|d, x, fc, st, tp| card::is_race_ctx(d, x, RACE_DRAGON, Some(fc), st, Some(tp)))),
        ],
    );
    // Cannot be used as Fusion Material
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e0:SetCode(EFFECT_CANNOT_BE_FUSION_MATERIAL)
    effect::set_code(d, e0, EFFECT_CANNOT_BE_FUSION_MATERIAL);
    // e0:SetValue(1)
    effect::set_value(d, e0, 1);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // This card loses 100 ATK for each of your banished cards
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetValue(function(e,cc) return Duel.GetFieldGroupCount(e:GetHandlerPlayer(),LOCATION_REMOVED,0)*-100 end)
    effect::set_value_fn(d, e1, effect_value(atk_value));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Banish 1 Special Summoned monster, also banish all monsters from the field with its same original Type
    // local e2a=Effect.CreateEffect(c)
    let e2a = effect::create_effect(d, c);
    // e2a:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2a, aux::stringid(ID, 0));
    // e2a:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2a, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2a:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2a, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e2a:SetCode(EVENT_CUSTOM+id)
    effect::set_code(d, e2a, EVENT_CUSTOM + ID);
    // e2a:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2a, LOCATION_MZONE);
    // e2a:SetCountLimit(1,id)
    effect::set_count_limit(d, e2a, 1, ID, 0, 0);
    // e2a:SetCondition(function() return not Duel.IsPhase(PHASE_DAMAGE) end)
    effect::set_condition(d, e2a, Some(condition(not_damage_phase)));
    // e2a:SetTarget(s.rmtg)
    effect::set_target(d, e2a, Some(cost_target(rmtg)));
    // e2a:SetOperation(s.rmop)
    effect::set_operation(d, e2a, Some(operation(rmop)));
    // c:RegisterEffect(e2a)
    card::register_effect(d, c, e2a, false);
    // local g=Group.CreateGroup()
    // g:KeepAlive()
    let g = CardSet::new();
    // e2a:SetLabelObject(g)
    effect::set_label_object(d, e2a, Value::Group(g));
    // Register summons
    // local e2b=Effect.CreateEffect(c)
    let e2b = effect::create_effect(d, c);
    // e2b:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2b, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2b:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2b, EVENT_SPSUMMON_SUCCESS);
    // e2b:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e2b, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e2b:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2b, LOCATION_MZONE);
    // e2b:SetLabelObject(e2a)
    effect::set_label_object(d, e2b, Value::Effect(e2a));
    // e2b:SetOperation(s.regop)
    effect::set_operation(d, e2b, Some(operation(regop)));
    // c:RegisterEffect(e2b)
    card::register_effect(d, c, e2b, false);
}
/// `function(e,cc) return Duel.GetFieldGroupCount(e:GetHandlerPlayer(),LOCATION_REMOVED,0)*-100 end`
fn atk_value(d: &mut Duel, e: EffectId) -> i64 {
    let p = effect::get_handler_player(d, e);
    duel::get_field_group_count(d, p, LOCATION_REMOVED, 0) as i64 * -100
}
/// `function() return not Duel.IsPhase(PHASE_DAMAGE) end`
fn not_damage_phase(d: &mut Duel, _a: &EffArgs) -> bool {
    !duel::is_phase(d, PHASE_DAMAGE)
}
/// `s.tgfilter(c,rc,e)`
fn tgfilter(d: &mut Duel, c: CardId, rc: Option<CardId>, e: Option<EffectId>) -> bool {
    // return c:IsLocation(LOCATION_MZONE) and (rc==c or (c:IsFaceup() and rc:IsOriginalRace(c:GetOriginalRace())))
    //     and c:IsAbleToRemove() and (not e or c:IsCanBeEffectTarget(e))
    if !card::is_location(d, c, LOCATION_MZONE) {
        return false;
    }
    let race_ok = if rc == Some(c) {
        true
    } else if card::is_faceup(d, c) {
        let race = card::get_original_race(d, c);
        card::is_original_race(d, rc.nil(), race)
    } else {
        false
    };
    race_ok
        && card::is_able_to_remove(d, c, None, None, None)
        && (e.is_none() || card::is_can_be_effect_target(d, c, e))
}
/// `s.tgfilter2(c,e)`
fn tgfilter2(d: &mut Duel, c: CardId, e: Option<EffectId>) -> bool {
    // return c:IsLocation(LOCATION_MZONE) and c:IsFaceup() and c:IsAbleToRemove() and (not e or c:IsCanBeEffectTarget(e))
    card::is_location(d, c, LOCATION_MZONE)
        && card::is_faceup(d, c)
        && card::is_able_to_remove(d, c, None, None, None)
        && (e.is_none() || card::is_can_be_effect_target(d, c, e))
}
/// `s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local rc=re:GetHandler()
    let _rc = effect::get_handler(d, a.re.nil()).nil();
    // local g=e:GetLabelObject():Filter(s.tgfilter2,nil,e)
    let lg = effect::get_label_object(d, a.e).as_group().cloned().nil();
    let g = group::filter(d, &lg, &|d, x| tgfilter2(d, x, Some(a.e)), NO_EX);
    // if chkc then return g:IsContains(chkc) and s.tgfilter(chkc,nil) end
    if let Some(chkc) = a.chkc() {
        return group::is_contains(&g, chkc) && tgfilter(d, chkc, None, None);
    }
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return group::get_count(&g) > 0;
    }
    // local tc=nil
    // if #g==1 then
    let tc: CardSet = if group::get_count(&g) == 1 {
        // tc=g:GetFirst()
        // Duel.SetTargetCard(tc)
        let first = group::get_first(d, &g).nil();
        duel::set_target_card(d, first);
        CardSet::from_card(first)
    } else {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
        // tc=g:Select(tp,1,1,nil)
        let sel = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
        // Duel.SetTargetCard(tc)
        duel::set_target_card(d, sel.clone());
        sel
    };
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,tc,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(tc), 1, a.tp, 0);
    true
}
/// `s.rmop(e,tp,eg,ep,ev,re,r,rp)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
        // local g=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsOriginalRace,tc:GetOriginalRace()),tp,LOCATION_MZONE,LOCATION_MZONE,tc)+tc
        let race = card::get_original_race(d, tc);
        let m = duel::get_matching_group(d, Some(&aux::faceup_filter(move |d, x| card::is_original_race(d, x, race))), a.tp, LOCATION_MZONE, LOCATION_MZONE, tc.into());
        let g = group::union(&m, &CardSet::from_card(tc));
        // Duel.Remove(g,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
/// `s.regop(e,tp,eg,ep,ev,re,r,rp)`
fn regop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if eg:IsContains(c) then return end
    let eg = a.eg.clone().nil();
    if group::is_contains(&eg, c) {
        return;
    }
    // local rc=re:GetHandler()
    let re = a.re.nil();
    let rc = effect::get_handler(d, re);
    // if not (re and re:IsActivated() and re:IsMonsterEffect() and rc) then return end
    let Some(rc) = rc else { return };
    if !effect::is_activated(d, re) || !effect::is_monster_effect(d, re) {
        return;
    }
    // local tg=eg:Filter(s.tgfilter,nil,rc)
    let tg = group::filter(d, &eg, &|d, x| tgfilter(d, x, Some(rc), None), NO_EX);
    // if #tg>0 then
    if group::get_count(&tg) > 0 {
        // for tc in tg:Iter() do
        for tc in group::iter(d, &tg) {
            // tc:RegisterFlagEffect(id,RESET_CHAIN,0,1)
            card::register_flag_effect(d, tc, ID, RESET_CHAIN, 0, 1, 0, 0);
        }
        // local g=e:GetLabelObject():GetLabelObject()
        let e2a = effect::get_label_object(d, a.e).as_effect().nil();
        let mut g = effect::get_label_object(d, e2a).as_group().cloned().nil();
        // if Duel.GetCurrentChain()==0 then g:Clear() end
        if duel::get_current_chain(d, false) == 0 {
            g = CardSet::new();
        }
        // g:Merge(tg)
        group::add_card(&mut g, &tg);
        // g:Remove(function(c) return c:GetFlagEffect(id)==0 end,nil)
        group::remove(d, &mut g, &|d, x| card::get_flag_effect(d, x, ID) == 0, NO_EX);
        // e:GetLabelObject():SetLabelObject(g)
        effect::set_label_object(d, e2a, Value::Group(g));
        // Duel.RaiseSingleEvent(e:GetHandler(),EVENT_CUSTOM+id,e,0,tp,tp,0)
        duel::raise_single_event(d, c, EVENT_CUSTOM + ID, Some(a.e), 0, a.tp, a.tp, 0);
    }
}
