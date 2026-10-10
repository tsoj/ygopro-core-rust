//! Bujinki Ahashima
//! Scripted by ahtelel, Naim and EerieCode
use super::prelude::*;

pub const ID: u32 = 71095768;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Duel.EnableGlobalFlag(GLOBALFLAG_DETACH_EVENT)
    duel::enable_global_flag(d, GLOBALFLAG_DETACH_EVENT);
    // Link summon
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link.AddProcedure(c,s.mfilter,2,nil,s.matcheck)
    link::add_procedure(d, c, Some(mat_filter(|d, x, _sc, _st, _tp| card::is_level_above(d, x, 1))), 2, None, Some(std::sync::Arc::new(matcheck)), None, None);
    // Cannot be link material
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_CANNOT_BE_LINK_MATERIAL)
    effect::set_code(d, e1, EFFECT_CANNOT_BE_LINK_MATERIAL);
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Xyz summon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(condition(spcon)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Destroy
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e3, CATEGORY_DESTROY);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_CHAINING)
    effect::set_code(d, e3, EVENT_CHAINING);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetCondition(s.descon)
    effect::set_condition(d, e3, Some(condition(descon)));
    // e3:SetTarget(s.destg)
    effect::set_target(d, e3, Some(cost_target(destg)));
    // e3:SetOperation(s.desop)
    effect::set_operation(d, e3, Some(operation(desop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // aux.DoubleSnareValidity(c,LOCATION_MZONE)
    aux::double_snare_validity(d, Some(c), LOCATION_MZONE, None);
    // aux.GlobalCheck(s,function() ... end)
    aux::global_check(d, ID, |d| {
        // s[0]=nil s[1]=nil s[2]=nil
        set_s_var(d, ID, 0, Value::Nil);
        set_s_var(d, ID, 1, Value::Nil);
        set_s_var(d, ID, 2, Value::Nil);
        // local ge1=Effect.CreateEffect(c)
        let ge1 = effect::create_effect(d, c);
        // ge1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // ge1:SetCode(EVENT_DETACH_MATERIAL)
        effect::set_code(d, ge1, EVENT_DETACH_MATERIAL);
        // ge1:SetOperation(s.checkop)
        effect::set_operation(d, ge1, Some(operation(checkop)));
        // Duel.RegisterEffect(ge1,0)
        duel::register_effect(d, ge1, 0);
    });
}
/// `Value` equality as Lua's `==` (nil equals only nil; numbers compare by value).
fn lua_eq(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Nil, Value::Nil) => true,
        (Value::Int(x), Value::Int(y)) => x == y,
        _ => false,
    }
}
/// `s.matcheck(g,lc,sumtype,tp)`: `g:GetClassCount(Card.GetLevel)==1`
fn matcheck(d: &mut Duel, g: &CardSet, _lc: CardId, _sumtype: u64, _tp: u8) -> bool {
    group::get_class_count(d, g, &|d, c| card::get_level(d, c) as i64) == 1
}
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsLinkSummoned()
    let c = effect::get_handler(d, a.e).nil();
    card::is_link_summoned(d, c)
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsLevelAbove(1) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_level_above(d, c, 1) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn xyzchk(d: &mut Duel, c: CardId, sg: &CardSet, minc: u16, maxc: u16, tp: u8) -> bool {
    // return c:IsXyzSummonable(nil,sg,minc,maxc) and Duel.GetLocationCountFromEx(tp,tp,sg,c)>0
    card::is_xyz_summonable(d, c, None, Some(sg.clone()), minc, maxc)
        && duel::get_location_count_from_ex(d, tp, Some(tp), (sg).into(), ygo_core::api::duel::ExtraCard::Card(c), None).0 > 0
}
fn spcheck(d: &mut Duel, sg: &CardSet, tp: u8) -> bool {
    // return sg:GetClassCount(Card.GetLocation)==#sg and sg:GetClassCount(Card.GetLevel)==1
    //     and Duel.IsExistingMatchingCard(s.xyzchk,tp,LOCATION_EXTRA,0,1,nil,sg,2,2,tp)
    group::get_class_count(d, sg, &|d, c| card::get_location(d, c) as i64) == sg.len()
        && group::get_class_count(d, sg, &|d, c| card::get_level(d, c) as i64) == 1
        && duel::is_existing_matching_card(d, Some(&|d, x| xyzchk(d, x, sg, 2, 2, tp)), tp, LOCATION_EXTRA, 0, 1, NO_EX)
}
/// The rescon form of `s.spcheck(sg,e,tp,mg)`: `(valid, stop)`.
fn spcheck_rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    (spcheck(d, sg, tp), false)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    let tp = a.tp;
    // if chk==0 then
    if a.chk == Some(0) {
        // local g=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_GRAVE|LOCATION_HAND,0,nil,e,tp)
        let g = duel::get_matching_group(d, Some(&|d, x| spfilter(d, x, e, tp)), tp, LOCATION_GRAVE | LOCATION_HAND, 0, NO_EX);
        // return Duel.IsPlayerCanSpecialSummonCount(tp,2)
        //     and not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
        //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>1
        //     and aux.SelectUnselectGroup(g,e,tp,2,2,s.spcheck,0)
        return duel::is_player_can_special_summon_count(d, tp, 2)
            && duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_none()
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 1
            && matches!(
                aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(&spcheck_rescon), 0, tp, None, None, None, false),
                Err(true)
            );
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,2,tp,LOCATION_HAND|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 2, tp, (LOCATION_HAND | LOCATION_GRAVE) as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    let tp = a.tp;
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then return end
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        return;
    }
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<2 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 < 2 {
        return;
    }
    // local g=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.spfilter),tp,LOCATION_GRAVE|LOCATION_HAND,0,nil,e,tp)
    let nv = aux::necro_valley_filter(|d, x| spfilter(d, x, e, tp));
    let g = duel::get_matching_group(d, Some(&nv), tp, LOCATION_GRAVE | LOCATION_HAND, 0, NO_EX);
    // local sg=aux.SelectUnselectGroup(g,e,tp,2,2,s.spcheck,1,tp,HINTMSG_SPSUMMON)
    let sg = aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(&spcheck_rescon), 1, tp, Some(HINTMSG_SPSUMMON as u64), None, None, false)
        .unwrap_or_default();
    // if #sg~=2 then return end
    if sg.len() != 2 {
        return;
    }
    // for tc in sg:Iter() do
    for tc in group::iter(d, &sg) {
        // Duel.SpecialSummonStep(tc,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon_step(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_DISABLE)
        effect::set_code(d, e1, EFFECT_DISABLE);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_DISABLE_EFFECT)
        effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
        // tc:RegisterEffect(e2)
        card::register_effect(d, tc, e2, false);
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
    // Duel.BreakEffect()
    duel::break_effect(d);
    // local xyzg=Duel.GetMatchingGroup(s.xyzchk,tp,LOCATION_EXTRA,0,nil,sg,2,2,tp)
    let xyzg = duel::get_matching_group(d, Some(&|d, x| xyzchk(d, x, &sg, 2, 2, tp)), tp, LOCATION_EXTRA, 0, NO_EX);
    // if #xyzg>0 then
    if !xyzg.is_empty() {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local xyz=xyzg:Select(tp,1,1,nil):GetFirst()
        let sel = group::select(d, &xyzg, tp, 1, 1, false, NO_EX).nil();
        let xyz = group::get_first(d, &sel).nil();
        // Duel.XyzSummon(tp,xyz,sg,nil,2,2)
        duel::xyz_summon(d, tp, xyz, Some(sg), None, 2, 2);
    }
}
/// `s.checkop(e,tp,eg,ep,ev,re,r,rp)`: records the chain link of a cost paid by an effect.
fn checkop(d: &mut Duel, a: &EffArgs) {
    // local cid=Duel.GetCurrentChain()
    let cid = duel::get_current_chain(d, false);
    // if cid>0 and r&REASON_COST==REASON_COST then
    if cid > 0 && (a.r & REASON_COST) == REASON_COST {
        let chain = cid as u8;
        // s[0],s[1]=Duel.GetChainInfo(cid,CHAININFO_CHAIN_ID),Duel.GetChainInfo(cid,CHAININFO_TRIGGERING_LOCATION)
        let id_info = duel::get_chain_info(d, chain, &[CHAININFO_CHAIN_ID]);
        let loc_info = duel::get_chain_info(d, chain, &[CHAININFO_TRIGGERING_LOCATION]);
        set_s_var(d, ID, 0, id_info[0].clone());
        set_s_var(d, ID, 1, loc_info[0].clone());
        // local seq=Duel.GetChainInfo(cid,CHAININFO_TRIGGERING_SEQUENCE)
        let mut seq = duel::get_chain_info(d, chain, &[CHAININFO_TRIGGERING_SEQUENCE])[0].int();
        // local te=Duel.GetChainInfo(cid,CHAININFO_TRIGGERING_EFFECT)
        let te = duel::get_chain_info(d, chain, &[CHAININFO_TRIGGERING_EFFECT])[0].as_effect().nil();
        // local tc,p=te:GetHandler(),e:GetHandler():GetControler()
        let tc = effect::get_handler(d, te).nil();
        let h = effect::get_handler(d, a.e).nil();
        let p = card::get_controler(d, h);
        // if tc:IsRelateToEffect(te) then
        if card::is_relate_to_effect(d, tc, te) {
            // if tc:IsControler(1-p) then seq=seq+16 end
            if card::is_controler(d, tc, 1 - p) {
                seq += 16;
            }
        } else if card::get_previous_controler(d, tc) == 1 - p {
            // if tc:GetPreviousControler()==1-p then seq=seq+16 end
            seq += 16;
        }
        // s[2]=seq
        set_s_var(d, ID, 2, Value::Int(seq));
    }
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local loc,seq=s[1],s[2]
    let loc = s_var(d, ID, 1);
    let seq = s_var(d, ID, 2);
    // if c:IsStatus(STATUS_BATTLE_DESTROYED) or not seq then return false end
    if card::is_status(d, c, STATUS_BATTLE_DESTROYED) || seq.is_nil() {
        return false;
    }
    // return Duel.GetChainInfo(ev,CHAININFO_CHAIN_ID)==s[0]
    //     and re:IsActiveType(TYPE_XYZ) and (loc&LOCATION_MZONE)~=0 and bit.extract(c:GetLinkedZone(),seq)~=0
    let chain_id = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_CHAIN_ID])[0].clone();
    if !lua_eq(&chain_id, &s_var(d, ID, 0)) {
        return false;
    }
    let re = a.re.nil();
    if !effect::is_active_type(d, re, TYPE_XYZ) {
        return false;
    }
    if loc.int() & LOCATION_MZONE as i64 == 0 {
        return false;
    }
    let linked = card::get_linked_zone(d, c, None);
    (linked >> seq.int()) & 1 != 0
}
fn desfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpellTrap()
    card::is_spell_trap(d, c)
}
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsOnField() and chkc:IsControler(1-tp) and s.desfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_controler(d, chkc, 1 - tp) && desfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.desfilter,tp,0,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&desfilter), tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,s.desfilter,tp,0,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_target(d, tp, Some(&desfilter), tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, 0, 0);
    true
}
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    // if tc and tc:IsRelateToEffect(e) then
    if let Some(tc) = duel::first_target(d) {
        if card::is_relate_to_effect(d, tc, a.e) {
            // Duel.Destroy(tc,REASON_EFFECT)
            duel::destroy(d, CardSet::from_card(tc), REASON_EFFECT, None, None);
        }
    }
}
